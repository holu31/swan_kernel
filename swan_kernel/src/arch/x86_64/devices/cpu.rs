use lazy_static::lazy_static;
use spin::Mutex;
use raw_cpuid::{CpuId, CpuIdReaderNative};

use crate::info;

lazy_static! {
    pub static ref CPUID: Mutex<CpuId<CpuIdReaderNative>> =
        Mutex::new(CpuId::new());
}

pub fn write_cpu_info() {
    let vendor_info = CPUID.lock().get_vendor_info().unwrap();
    let proccessor_brand_string = CPUID.lock().get_processor_brand_string()
        .unwrap();
    let cpu_frequency_info = CPUID.lock().get_processor_frequency_info();

    info!("{} {} ({:?})",
        vendor_info,
        proccessor_brand_string.as_str().trim(),
        cpu_frequency_info);

    if let Some(cache_info) = CPUID.lock().get_cache_info() {
        info!("CPU cache {:?}\n", cache_info);
    }
}