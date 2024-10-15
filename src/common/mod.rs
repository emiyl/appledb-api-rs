pub mod bypass;
pub mod device_group;
pub mod device;
pub mod file;
pub mod jailbreak;
pub mod json;
pub mod os;

use serde_json::{json, Value};
use std::{fs, io::Write, os::unix::fs::FileExt};
use walkdir::WalkDir;

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

#[derive(PartialEq)]
pub enum EntryType {
    Os,
    Device,
    DeviceGroup,
    Jailbreak,
    Bypass
}

pub struct OutputEntry {
    json: String,
    key: String,
}

pub struct OutputFormat {
    value_vec: Vec<Value>,
    pub file_count: u32,
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
        EntryType::Os => os::process_entry(json_value, output.value_vec, output_dir, extra_input_value),
        EntryType::Device => device::process_entry(json_value, output.value_vec, extra_input_value),
        EntryType::DeviceGroup => device_group::process_entry(json_value, output.value_vec),
        EntryType::Jailbreak => jailbreak::process_entry(json_value, output.value_vec),
        EntryType::Bypass => bypass::process_entry(json_value, output.value_vec),
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

pub fn create_entries(
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
        #[cfg(feature = "api")]
        EntryType::Os => os::finalise_entry(&output_dir_string, output),
        #[cfg(feature = "adb_web")]
        EntryType::Os => os::finalise_entry(&output_dir_string, output),
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