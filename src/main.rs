mod device_file;
mod device_group;
mod file;
mod json;
mod os_file;
use std::{fs, io::Write, os::unix::fs::FileExt};
use walkdir::WalkDir;

#[derive(PartialEq)]
enum EntryType {
    OsEntry,
    DeviceEntry,
    DeviceGroup,
}

struct OutputEntry {
    json: String,
    key: String,
}

fn main() {
    let now = std::time::Instant::now();
    let mut file_count: u32 = 0;
    file_count += create_entries(
        EntryType::OsEntry,
        "./appledb/osFiles/".to_string(),
        "./out/firmware/".to_string(),
    ) + create_entries(
        EntryType::DeviceEntry,
        "./appledb/deviceFiles/".to_string(),
        "./out/device/key/".to_string(),
    ) + create_entries(
        EntryType::DeviceGroup,
        "./appledb/deviceGroupFiles/".to_string(),
        "./out/device/".to_string(),
    );
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
}

fn create_entries(entry_type: EntryType, input_dir: String, output_dir: String) -> u32 {
    let mut file_count: u32 = 0;
    file::mkdir(&output_dir).expect("Failed to create directory");

    let main_index_json_path_array =
        ["main.json", "index.json"].map(|str| [&output_dir.as_str(), str].concat());
    let main_index_json_file_vec = main_index_json_path_array.map(|path| {
        let mut ret = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();
        ret.write_all("[".as_bytes())
            .expect("Failed to write to main/index json file");
        ret
    });
    let entry_list = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            if entry.path().extension().map_or(false, |ext| ext == "json") {
                Some(entry)
            } else {
                None
            }
        });

    let mut os_str_vec: Vec<String> = Vec::new();

    for entry in entry_list {
        let path = entry.path().to_str().unwrap();
        let json_string = file::open_file_to_string(path);
        let json_value = json::parse_json(&json_string);

        let output_entry_list = match entry_type {
            EntryType::OsEntry => {
                let os_entry_vec = os_file::get_os_entry_vec_from_path(json_value);

                let mut output_entry_vec: Vec<OutputEntry> = Vec::new();
                for os_entry in os_entry_vec {
                    let output_entry = OutputEntry {
                        json: serde_json::to_string(&os_entry)
                            .expect("Failed to convert struct to JSON"),
                        key: os_entry.key.replace(';', "/"),
                    };

                    // OsEntry needs the firmware/<os_str>/<"main"|"index">.json files
                    // Use os_str_vec to keep track of which os_str files have been created
                    // Since the script appends to files, we need to know which files have already been created or not
                    let os_str = os_entry.osStr;
                    let os_str_vec_contains = os_str_vec.contains(&os_str);
                    file_count += os_file::write_os_str_main_index_json_files(
                        &output_dir,
                        &os_str,
                        &output_entry.json,
                        &output_entry.key,
                        !os_str_vec_contains,
                    );
                    if !os_str_vec_contains {
                        os_str_vec.push(os_str.to_owned())
                    };

                    output_entry_vec.push(output_entry);
                }
                output_entry_vec
            }
            EntryType::DeviceEntry => {
                let device_entry = device_file::create_device_entry_from_json(&json_value);
                vec![OutputEntry {
                    json: serde_json::to_string(&device_entry)
                        .expect("Failed to convert struct to JSON"),
                    key: device_entry.key.to_owned(),
                }]
            }
            EntryType::DeviceGroup => {
                let device_group = device_group::create_device_group_from_json(&json_value);
                vec![OutputEntry {
                    json: serde_json::to_string(&device_group)
                        .expect("Failed to convert struct to JSON"),
                    key: device_group.key.to_owned(),
                }]
            }
        };

        for output_entry in output_entry_list {
            let output_path = [output_dir.as_str(), &output_entry.key, ".json"].concat();
            file::write_string_to_file(&output_path, &output_entry.json)
                .expect("Failed to write device JSON");
            file_count += 1;

            let main_index_json_file_buf = vec![
                [output_entry.json, ",".to_string()].concat(),
                ["\"".to_string(), output_entry.key, "\",".to_string()].concat(),
            ];

            for (i, mut file) in main_index_json_file_vec.iter().enumerate() {
                file.write_all(main_index_json_file_buf[i].as_bytes())
                    .expect("Failed to write to main/index json file")
            }
        }
    }

    if entry_type == EntryType::OsEntry {
        os_file::finalise_os_str_main_index_json_files(&output_dir, os_str_vec);
    };

    for file in main_index_json_file_vec {
        let len = file.metadata().unwrap().len();
        let offset = if len > 1 { len - 1 } else { len };
        file.write_at("]\n".as_bytes(), offset)
            .expect("Failed to write to device main/index json file");
        file_count += 1;
    }

    file_count
}
