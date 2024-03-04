use std::{fs, io::Write, os::unix::fs::FileExt};

use crate::{file, json};
use serde::Serialize;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;
use walkdir::WalkDir;

#[derive(FieldNamesAsArray, Default, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct DeviceEntry {
    name: String,
    key: String,
    r#type: String,
    identifier: Vec<String>,
    model: Vec<String>,
    board: Vec<String>,
    released: Vec<String>,
    soc: Vec<String>,
    arch: String,
    internal: bool,
    alias: Vec<String>,
    info: Value,
    iBridge: String,
    group: bool,
    windowsStoreId: String,
}

fn create_device_entry_from_json(json: &Value) -> DeviceEntry {
    let mut entry: DeviceEntry = Default::default();
    let json_field_list = json::get_object_field_list(json);

    let identifier = json::get_vec_from_string_or_string_vec(json, "identifier");

    for field in DeviceEntry::FIELD_NAMES_AS_ARRAY {
        match field {
            "name" => entry.name = json::get_string(json, field),
            "key" => {
                entry.key = if json_field_list.contains(&&"key".to_string()) {
                    json::get_string(json, field)
                } else if !identifier.is_empty() {
                    identifier[0].clone()
                } else {
                    json::get_string(json, "name")
                }
            }
            "r#type" => entry.r#type = json::get_string(json, "type"),
            "identifier" => entry.identifier = identifier.clone(),
            "model" => entry.model = json::get_vec_from_string_or_string_vec(json, field),
            "board" => entry.board = json::get_vec_from_string_or_string_vec(json, field),
            "released" => entry.released = json::get_vec_from_string_or_string_vec(json, field),
            "soc" => entry.soc = json::get_vec_from_string_or_string_vec(json, field),
            "arch" => entry.arch = json::get_string(json, field),
            "internal" => entry.internal = json::get_bool(json, field),
            "alias" => entry.alias = json::get_vec_from_string_or_string_vec(json, field),
            "info" => entry.info = json::get_object(json, field),
            "iBridge" => entry.iBridge = json::get_string(json, field),
            "group" => entry.group = json::get_bool(json, field),
            "windowsStoreId" => entry.windowsStoreId = json::get_string(json, field),
            _ => println!("Unknown key"),
        }
    }

    entry
}

fn get_device_entry_from_path(file_path: &str) -> DeviceEntry {
    let json_string = file::open_file_to_string(file_path);
    let json_value = json::parse_json(&json_string);
    create_device_entry_from_json(&json_value)
}

pub fn create_devices() -> u32 {
    let mut file_count: u32 = 0;
    let out_dir = "./out/device/";
    file::mkdir(out_dir.to_string()).expect("Failed to create directory ./out/firmware");

    let main_index_json_path_array = ["main.json", "index.json"].map(|str| [out_dir, str].concat());
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
    let entry_list = WalkDir::new("./appledb/deviceFiles")
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
        let device_entry = get_device_entry_from_path(path);
        let out_json =
            serde_json::to_string(&device_entry).expect("Failed to convert struct to JSON");

        let output_path = [out_dir, &device_entry.key, ".json"].concat();
        file::write_string_to_file(&output_path, &out_json).expect("Failed to write device JSON");
        file_count += 1;

        let main_index_json_file_buf = vec![
            [out_json, ",".to_string()].concat(),
            ["\"".to_string(), device_entry.clone().key, "\"".to_string()].concat(),
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
