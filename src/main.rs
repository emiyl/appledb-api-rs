mod device_file;
mod file;
mod json;
mod os_file;
use std::{fs, io::Write, os::unix::fs::FileExt};
use walkdir::WalkDir;

enum EntryType {
    DeviceEntry,
}

struct OutputEntry {
    json: String,
    key: String,
}

fn main() {
    let now = std::time::Instant::now();
    let mut file_count: u32 = 0;
    file_count += os_file::create_firmware()
        + create_entries(
            EntryType::DeviceEntry,
            "./appledb/deviceFiles/".to_string(),
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

    for entry in entry_list {
        let path = entry.path().to_str().unwrap();
        let json_string = file::open_file_to_string(path);
        let json_value = json::parse_json(&json_string);

        let output_entry = match entry_type {
            EntryType::DeviceEntry => {
                let device_entry = device_file::create_device_entry_from_json(&json_value);
                OutputEntry {
                    json: serde_json::to_string(&device_entry)
                        .expect("Failed to convert struct to JSON"),
                    key: device_entry.key.clone(),
                }
            }
        };

        let output_path = [output_dir.as_str(), &output_entry.key, ".json"].concat();
        file::write_string_to_file(&output_path, &output_entry.json)
            .expect("Failed to write device JSON");
        file_count += 1;

        let main_index_json_file_buf = vec![
            [output_entry.json, ",".to_string()].concat(),
            ["\"".to_string(), output_entry.key, "\"".to_string()].concat(),
        ];

        for (i, mut file) in main_index_json_file_vec.iter().enumerate() {
            file.write_all(main_index_json_file_buf[i].as_bytes())
                .expect("Failed to write to main/index json file")
        }
    }

    for file in main_index_json_file_vec {
        let len = file.metadata().unwrap().len();
        let offset = if len > 1 { len - 1 } else { len };
        file.write_at("]\n".as_bytes(), offset)
            .expect("Failed to write to device main/index json file");
        file_count += 1;
    }

    file_count
}
