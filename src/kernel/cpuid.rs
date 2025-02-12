use crate::print;
use raw_cpuid::CpuId;

pub fn init() {
    let cpuid = CpuId::new();

    if let Some(vendor_info) = cpuid.get_vendor_info() {
        print!("CPU vendor: {}\n", vendor_info);
    } else {
        print!("Failed to retrieve CPU vendor information.\n");
    }

    if let Some(processor_brand_string) = cpuid.get_processor_brand_string() {
        print!("CPU brand: {}\n", processor_brand_string.as_str().trim());
    } else {
        print!("Failed to retrieve CPU brand information.\n");
    }

    if let Some(info) = cpuid.get_processor_frequency_info() {
        let frequency = info.processor_base_frequency();
        if frequency > 0 {
            print!("CPU clock speed: {} MHz\n", frequency);
        } else {
            print!("CPU clock speed: Unknown\n");
        }
    } else {
        print!("Failed to retrieve CPU clock speed information.\n");
    }
}
