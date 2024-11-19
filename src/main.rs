mod common;

use serde_json::json;
use crate::common::{create_entries, EntryType};

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let now = std::time::Instant::now();
    let os_entry = create_entries(
        EntryType::Os,
        "./appledb/osFiles/",
        "./out/firmware/",
        &json!([])
    );
    let device_entry = create_entries(
        EntryType::Device,
        "./appledb/deviceFiles/",
        "./out/device/key/",
        &json!([])
    );
    let device_group_entry = create_entries(
        EntryType::DeviceGroup,
        "./appledb/deviceGroupFiles/",
        "./out/device/group/",
        &json!([])
    );
    let jailbreak_entry = create_entries(
        EntryType::Jailbreak,
        "./tmp/jailbreak/",
        "./out/jailbreak/",
        &json!([])
    );
    let bypass_entry = create_entries(
        EntryType::Bypass, 
        "./appledb/bypassApps",
        "./out/bypass/",
        &json!([])
    );

    let file_count = 
        os_entry.file_count +
        device_entry.file_count +
        device_group_entry.file_count +
        jailbreak_entry.file_count +
        bypass_entry.file_count;
    
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);

    let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
    println!("PEAK_ALLOC: {:.2?} MB", peak_mem);
}