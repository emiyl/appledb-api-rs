#[path = "../../src/common/mod.rs"]
mod common;
mod adbweb_os;
mod adbweb_device;
mod adbweb_device_group;

use serde_json::Value;
use std::collections::BTreeMap;
use peak_alloc::PeakAlloc;

use std::process::Command;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

use crate::common::{create_entries, EntryType, file, json};

fn main() {
    let now = std::time::Instant::now();

    let os_now = std::time::Instant::now();
    let device_group_main_json_string = file::open_file_to_string("./out/device/group/main.json");
    let device_group_main_json_value = json::parse_json(&device_group_main_json_string);

    let os_entry = create_entries(
        EntryType::Os,
        "./appledb/osFiles/",
        "./out/adbweb/firmware/",
        &device_group_main_json_value
    );

    println!("OsEntry: Processed {} files in {:.2?}", os_entry.file_count, os_now.elapsed());

    let device_now = std::time::Instant::now();
    let os_main_json_string = file::open_file_to_string("./out/firmware/main.json");
    let os_main_json_value = json::parse_json(&os_main_json_string);

    let device_entry = create_entries(
        EntryType::Device,
        "./appledb/deviceFiles/",
        "./out/adbweb/device/key/",
        &os_main_json_value
    );

    println!("DeviceEntry: Processed {} files in {:.2?}", device_entry.file_count, device_now.elapsed());

    #[cfg(all(unix, feature = "node_fix_json"))]
    Command::new("sh")
        .arg("-c")
        .arg("node fix_json.js ./out/adbweb/device/key/main.json")
        .output()
        .expect("failed to execute process");

    let device_group_now = std::time::Instant::now();
    let device_main_json_string = file::open_file_to_string("./out/adbweb/device/key/main.json");
    let device_main_json_value = json::parse_json(&device_main_json_string);
    let device_main_iter = device_main_json_value.as_array().unwrap().iter();
    let mut device_main_map: BTreeMap<String, Value> = BTreeMap::new();

    for device in device_main_iter {
        device_main_map.insert(json::get_string(device, "key"), device.clone());
    }

    device_main_map.insert("os_main_json_value".to_string(), os_main_json_value);
    let device_main_map_value = serde_json::to_value(device_main_map).unwrap();
    
    let device_group_entry = create_entries(
        EntryType::DeviceGroup,
        "./appledb/deviceGroupFiles/",
        "./out/adbweb/device/group/",
        &device_main_map_value
    );

    println!("DeviceGroupEntry: Processed {} files in {:.2?}", device_group_entry.file_count, device_group_now.elapsed());

    let file_count = os_entry.file_count + device_entry.file_count + device_group_entry.file_count;
    
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);

    let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
    println!("PEAK_ALLOC: {:.2?} MB", peak_mem);*/
}