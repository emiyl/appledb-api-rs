#[path = "../../src/common/mod.rs"]
mod common;
mod adbweb_os;
mod adbweb_device;

use crate::common::{create_entries, EntryType, file, json};

fn main() {
    let now = std::time::Instant::now();

    let device_group_main_json_string = file::open_file_to_string("./out/device/group/main.json");
    let device_group_main_json_value = json::parse_json(&device_group_main_json_string);

    let os_entry = create_entries(
        EntryType::Os,
        "./appledb/osFiles/",
        "./out/adbweb/firmware/",
        &device_group_main_json_value
    );

    let os_main_json_string = file::open_file_to_string("./out/firmware/main.json");
    let os_main_json_value = json::parse_json(&os_main_json_string);

    let device_entry = create_entries(
        EntryType::Device,
        "./appledb/deviceFiles/",
        "./out/adbweb/device/key/",
        &os_main_json_value
    );

    let file_count = os_entry.file_count + device_entry.file_count;
    
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}