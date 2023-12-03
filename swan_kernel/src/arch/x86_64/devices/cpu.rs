use raw_cpuid::CpuId;

use crate::println;

pub fn write_cpu_info() {
    let cpuid = CpuId::new();

    let vendor_info = cpuid.get_vendor_info().unwrap();
    let proccessor_brand_string = cpuid.get_processor_brand_string()
        .unwrap();
    let cpu_frequency_info = cpuid.get_processor_frequency_info();

    println!("{} {} ({:?})",
        vendor_info,
        proccessor_brand_string.as_str().trim(),
        cpu_frequency_info);

    if let Some(cache_info) = cpuid.get_cache_info() {
        println!("CPU cache {:?}\n", cache_info);
    }
}