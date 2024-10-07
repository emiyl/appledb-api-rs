mod device;
mod device_group;
mod file;
mod json;
mod os;
mod jailbreak;
mod bypass;
mod adb_web;
use serde_json::{json, Value};
use std::{fs, io::Write, os::unix::fs::FileExt};
use walkdir::WalkDir;

#[derive(PartialEq)]
enum EntryType {
    Os,
    Device,
    DeviceGroup,
    Jailbreak,
    Bypass,

    OsADBWeb
}

struct OutputEntry {
    json: String,
    key: String,
}

struct OutputFormat {
    value_vec: Vec<Value>,
    file_count: u32,
}

macro_rules! filter_dir_recurse {
    ($dir:expr,$extension:expr) => {
        {
            WalkDir::new($dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter_map(|entry| {
                    if entry
                        .path()
                        .extension()
                        .map_or(false, |ext| ext == $extension)
                    {
                        Some(entry)
                    } else {
                        None
                    }
                })
        }
    };
}

fn create_main_index_json_file(output_dir: &str) -> [fs::File; 2] {
    let main_index_json_path_array =
        ["main.json", "index.json"].map(|str| [&output_dir, str].concat());
    main_index_json_path_array.map(|path| {
        let mut ret = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        ret.write_all("[".as_bytes())
            .expect("Failed to write to main/index json file");
        ret
    })
}

fn finalise_main_index_json_file(main_index_json_file_vec: &[fs::File; 2]) -> u32 {
    let mut file_count: u32 = 0;
    for file in main_index_json_file_vec {
        let len = file.metadata().unwrap().len();
        let offset = if len > 1 { len - 1 } else { len };
        file.write_at("]\n".as_bytes(), offset)
            .expect("Failed to write to device main/index json file");
        file_count += 1;
    }
    file_count
}

fn write_entry(
    entry_type: &EntryType,
    json_value: Value,
    mut output: OutputFormat,
    output_dir: &String,
    main_index_json_file_vec: &[fs::File; 2],
    extra_input_value: &Value
) -> OutputFormat {
    let output_entry_tuple = match entry_type {
        EntryType::Os => os::process_entry(json_value, output.value_vec, output_dir, extra_input_value, false),
        EntryType::Device => device::process_entry(json_value, output.value_vec),
        EntryType::DeviceGroup => device_group::process_entry(json_value, output.value_vec),
        EntryType::Jailbreak => jailbreak::process_entry(json_value, output.value_vec),
        EntryType::Bypass => bypass::process_entry(json_value, output.value_vec),

        EntryType::OsADBWeb => os::process_entry(json_value, output.value_vec, output_dir, extra_input_value, true)
    };

    let output_entry_list = output_entry_tuple.0;
    output.value_vec = output_entry_tuple.1.value_vec;
    output.file_count += output_entry_tuple.1.file_count;

    for output_entry in output_entry_list {
        let output_path = [output_dir.as_str(), &output_entry.key, ".json"].concat();
        file::write_string_to_file(&output_path, &output_entry.json)
            .expect("Failed to write device JSON");
        output.file_count += 1;

        let main_index_json_file_buf = vec![
            [output_entry.json, ",".to_string()].concat(),
            ["\"".to_string(), output_entry.key, "\",".to_string()].concat(),
        ];

        for (i, mut file) in main_index_json_file_vec.iter().enumerate() {
            file.write_all(main_index_json_file_buf[i].as_bytes())
                .expect("Failed to write to main/index json file")
        }
    }

    output
}

fn create_entries(
    entry_type: EntryType,
    input_dir: &str,
    output_dir: &str,
    extra_input_value: &Value
) -> OutputFormat {
    let output_dir_string = output_dir.to_string();
    let input_vec = Vec::new();

    file::mkdir(&output_dir_string).expect("Failed to create directory");

    let mut output = OutputFormat {
        value_vec: Vec::new(),
        file_count: 0,
    };
    let main_index_json_file_array = create_main_index_json_file(&output_dir);
    let entry_list = filter_dir_recurse!(input_dir, "json");

    for entry in entry_list {
        let path = entry.path().to_str().unwrap();
        let json_string = file::open_file_to_string(path);
        let json_value = json::parse_json(&json_string);

        output = write_entry(
            &entry_type,
            json_value,
            output,
            &output_dir_string,
            &main_index_json_file_array,
            extra_input_value
        );
    }

    output = match entry_type {
        EntryType::Os => os::finalise_entry(&output_dir_string, output),
        EntryType::OsADBWeb => os::finalise_entry(&output_dir_string, output),
        EntryType::DeviceGroup => device_group::finalise_entry(
            &output_dir_string,
            &input_vec,
            output,
            &main_index_json_file_array,
        ),
        _ => output
    };

    output.file_count += finalise_main_index_json_file(&main_index_json_file_array);

    output
}

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

    let mut file_count = 
        os_entry.file_count +
        device_entry.file_count +
        device_group_entry.file_count +
        jailbreak_entry.file_count +
        bypass_entry.file_count;

    let device_group_main_json_string = file::open_file_to_string("./out/device/group/main.json");
    println!("here!");
    let device_group_main_json_value = json::parse_json(&device_group_main_json_string);
    println!("here too!");

    let os_adbweb_entry = create_entries(
        EntryType::OsADBWeb,
        "./appledb/osFiles/",
        "./out/adbweb/firmware/",
        &device_group_main_json_value
    );

    file_count += os_adbweb_entry.file_count;
    
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}