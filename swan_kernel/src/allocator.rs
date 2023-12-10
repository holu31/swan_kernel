use core::{cmp::*, slice::SliceIndex};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use x86_64::structures::paging::OffsetPageTable;
use linked_list_allocator::LockedHeap;
use spin::Mutex;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: u64 = 0x4444_4444_0000;

pub fn init_heap() -> Result<(), MapToError<Size4KiB>> {
    let mapper = memory::mapper();
    let mut frame_allocator = memory::frame_allocator();

    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_size = min(memory::memory_size(), max_memory());
    let heap_end = heap_start + heap_size - 1u64;

    let page_range = {
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, &mut frame_allocator)?.flush()
        };
    }

    unsafe {
        ALLOCATOR.lock().init(heap_start.as_mut_ptr(), heap_size as usize);
    }

    ok!("Kernel heap succesful initialization");
    Ok(())
}

pub fn alloc_pages(mapper: &mut OffsetPageTable, addr: u64, size: usize) -> Result<(), ()> {
    let mut frame_allocator = memory::frame_allocator();
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
    let pages = {
        let start_page = Page::containing_address(VirtAddr::new(addr));
        let end_page = Page::containing_address(VirtAddr::new(addr + (size as u64) - 1));
        Page::range_inclusive(start_page, end_page)
    };
    for page in pages {
        if let Some(frame) = frame_allocator.allocate_frame() {
            unsafe {
                if let Ok(mapping) = mapper.map_to(page, frame, flags, &mut frame_allocator) {
                    mapping.flush();
                } else {
                    return Err(());
                }
            }
        } else {
            return Err(());
        }
    }
    Ok(())
}

use x86_64::structures::paging::page::PageRangeInclusive;

use crate::{memory, ok};

pub fn free_pages(mapper: &mut OffsetPageTable, addr: u64, size: usize) {
    let pages: PageRangeInclusive<Size4KiB> = {
        let start_page = Page::containing_address(VirtAddr::new(addr));
        let end_page = Page::containing_address(VirtAddr::new(addr + (size as u64) - 1));
        Page::range_inclusive(start_page, end_page)
    };
    for page in pages {
        if let Ok((_frame, mapping)) = mapper.unmap(page) {
            mapping.flush();
        }
    }
}

#[derive(Clone)]
pub struct PhysBuf {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl PhysBuf {
    pub fn new(len: usize) -> Self {
        Self::from(vec![0; len])
    }

    fn from(vec: Vec<u8>) -> Self {
        let buffer_len = vec.len() - 1;
        let memory_len = phys_addr(&vec[buffer_len]) - phys_addr(&vec[0]);
        if buffer_len == memory_len as usize {
            Self { buf: Arc::new(Mutex::new(vec)) }
        } else {
            Self::from(vec.clone())
        }
    }

    pub fn addr(&self) -> u64 {
        phys_addr(&self.buf.lock()[0])
    }
}

pub fn phys_addr(ptr: *const u8) -> u64 {
    let virt_addr = VirtAddr::new(ptr as u64);
    let phys_addr = memory::virt_to_phys(virt_addr).unwrap();
    phys_addr.as_u64()
}

impl<I: SliceIndex<[u8]>> Index<I> for PhysBuf {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for PhysBuf {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl core::ops::Deref for PhysBuf {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        let vec = self.buf.lock();
        unsafe { alloc::slice::from_raw_parts(vec.as_ptr(), vec.len()) }
    }
}

impl core::ops::DerefMut for PhysBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        let mut vec = self.buf.lock();
        unsafe { alloc::slice::from_raw_parts_mut(vec.as_mut_ptr(), vec.len()) }
    }
}

pub fn memory_size() -> usize {
    ALLOCATOR.lock().size()
}

pub fn memory_used() -> usize {
    ALLOCATOR.lock().used()
}

pub fn memory_free() -> usize {
    ALLOCATOR.lock().free()
}

fn max_memory() -> u64 {
    option_env!("MEMORY").unwrap_or("1").parse::<u64>().unwrap() << 20 // MB
}