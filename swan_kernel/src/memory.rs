use x86_64::{
    VirtAddr, PhysAddr,
    structures::paging::{Page, PhysFrame, Mapper, Size4KiB,
        FrameAllocator,PageTable, OffsetPageTable, Translate}, registers::control::Cr3
};
use bootloader::bootinfo::{BootInfo, MemoryMap, MemoryRegionType};
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::instructions::interrupts;
use crate::{info, allocator, ok};

pub static mut PHYS_MEM_OFFSET: Option<u64> = None;
pub static mut MEMORY_MAP: Option<&MemoryMap> = None;
pub static mut MAPPER: Option<OffsetPageTable<'static>> = None;

pub static MEMORY_SIZE: AtomicU64 = AtomicU64::new(0);

pub fn init(boot_info: &'static BootInfo) {
    interrupts::without_interrupts(|| {
        let mut memory_size = 0;
        for region in boot_info.memory_map.iter() {
            let start_addr = region.range.start_addr();
            let end_addr = region.range.end_addr();
            memory_size += end_addr - start_addr;
            info!("MEM [{:#016X}-{:#016X}] {:?}", start_addr, end_addr - 1, region.region_type);
        }
        info!("MEMORY {} KB", memory_size >> 10);
        MEMORY_SIZE.store(memory_size, Ordering::Relaxed);

        let phys_mem_offset = boot_info.physical_memory_offset;

        unsafe { PHYS_MEM_OFFSET.replace(phys_mem_offset) };
        unsafe { MEMORY_MAP.replace(&boot_info.memory_map) };
        unsafe { MAPPER.replace(OffsetPageTable::new(active_page_table(), VirtAddr::new(phys_mem_offset))) };

        ok!("Memory manager initialization sucessful");
        allocator::init_heap().expect("heap initialization failed.");
    });
}

pub fn mapper() -> &'static mut OffsetPageTable<'static> {
    unsafe { MAPPER.as_mut().unwrap() }
}

pub fn frame_allocator() -> BootInfoFrameAllocator {
    unsafe { BootInfoFrameAllocator::init(MEMORY_MAP.unwrap()) }
}

pub fn memory_size() -> u64 {
    MEMORY_SIZE.load(Ordering::Relaxed)
}

pub fn phys_to_virt(addr: PhysAddr) -> VirtAddr {
    let phys_mem_offset = unsafe { PHYS_MEM_OFFSET.unwrap() };
    VirtAddr::new(addr.as_u64() + phys_mem_offset)
}

pub fn virt_to_phys(addr: VirtAddr) -> Option<PhysAddr> {
    mapper().translate_addr(addr)
}

pub unsafe fn active_page_table() -> &'static mut PageTable {
    let (frame, _) = Cr3::read();
    let phys_addr = frame.start_address();
    let virt_addr = phys_to_virt(phys_addr);
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    &mut *page_table_ptr // unsafe
}

pub unsafe fn create_page_table(frame: PhysFrame) -> &'static mut PageTable {
    let phys_addr = frame.start_address();
    let virt_addr = phys_to_virt(phys_addr);
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    &mut *page_table_ptr // unsafe
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr)
    -> Option<PhysAddr>
{
    use x86_64::structures::paging::page_table::FrameError;

    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}