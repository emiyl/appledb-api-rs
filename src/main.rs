mod device_file;
mod device_group;
mod file;
mod json;
mod os_file;
use serde_json::{json, Value};
use std::{fs, io::Write, os::unix::fs::FileExt};
use walkdir::WalkDir;

macro_rules! filter_dir_recurse {
    // macth like arm for macro
    ($dir:expr,$extension:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
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
enum EntryType {
    Os,
    Device,
    DeviceGroup,
}

struct OutputEntry {
    json: String,
    key: String,
}

fn main() {
    let now = std::time::Instant::now();
    let mut file_count: u32 = 0;
    let os_entry = create_entries(
        EntryType::Os,
        "./appledb/osFiles/".to_string(),
        "./out/firmware/".to_string(),
        Vec::new(),
    );
    let device_entry = create_entries(
        EntryType::Device,
        "./appledb/deviceFiles/".to_string(),
        "./out/device/key/".to_string(),
        Vec::new(),
    );
    let device_group_entry = create_entries(
        EntryType::DeviceGroup,
        "./appledb/deviceGroupFiles/".to_string(),
        "./out/device/".to_string(),
        device_entry.0,
    );
    file_count += os_entry.1 + device_entry.1 + device_group_entry.1;
    let elapsed = now.elapsed();
    println!("Processed {} files in {:.2?}", file_count, elapsed);
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
    mut output_vec: Vec<Value>,
    output_dir: &String,
    main_index_json_file_vec: &[fs::File; 2],
) -> (Vec<Value>, u32) {
    let mut file_count: u32 = 0;

    let output_entry_tuple = match entry_type {
        EntryType::Os => os_file::process_entry(json_value, output_vec, output_dir),
        EntryType::Device => device_file::process_entry(json_value, output_vec),
        EntryType::DeviceGroup => device_group::process_entry(json_value, output_vec),
    };

    let output_entry_list = output_entry_tuple.0;
    output_vec = output_entry_tuple.1;
    file_count += output_entry_tuple.2;

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

    (output_vec, file_count)
}

fn create_entries(
    entry_type: EntryType,
    input_dir: String,
    output_dir: String,
    input_vec: Vec<Value>,
) -> (Vec<Value>, u32) {
    file::mkdir(&output_dir).expect("Failed to create directory");

    let mut file_count: u32 = 0;
    let main_index_json_file_array = create_main_index_json_file(&output_dir);
    let entry_list = filter_dir_recurse!(input_dir, "json");
    let mut output_vec: Vec<Value> = Vec::new();

    for entry in entry_list {
        let path = entry.path().to_str().unwrap();
        let json_string = file::open_file_to_string(path);
        let json_value = json::parse_json(&json_string);

        let tuple = write_entry(
            &entry_type,
            json_value,
            output_vec,
            &output_dir,
            &main_index_json_file_array,
        );

        output_vec = tuple.0;
        file_count += tuple.1;
    }

    let finalise_tuple = match entry_type {
        EntryType::Os => os_file::finalise_entry(&output_dir, &output_vec),
        EntryType::Device => device_file::finalise_entry(&output_vec),
        EntryType::DeviceGroup => device_group::finalise_entry(
            &output_dir,
            &input_vec,
            &output_vec,
            &main_index_json_file_array,
        ),
    };
    output_vec = finalise_tuple.0;
    file_count += finalise_tuple.1;

    file_count += finalise_main_index_json_file(&main_index_json_file_array);

    (output_vec, file_count)
}
