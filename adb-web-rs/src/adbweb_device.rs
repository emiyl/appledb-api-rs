use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;
use serde::Serialize;
use crate::common::{json, device};

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone, Debug)]]
    #[allow(non_snake_case)]
    pub struct DeviceADBWebEntry {
        name: String,
        pub key: String,
        r#type: String,
        identifier: Vec<String>,
        model: Vec<String>,
        board: Vec<String>,
        released: Vec<String>,
        soc: Vec<String>,
        arch: Vec<String>,
        firmwares: Vec<pub struct DeviceADBWebEntryOsEntry {
            os_str: String,
            version: String,
            build: String,
            key: String,
            released: String,
            rc: bool,
            beta: bool,
            rsr: bool,
            internal: bool,
            preinstalled: Vec<String>,
            sources: Vec<pub struct DeviceADBWebEntryOsEntrySource {
                r#type: String,
                device_map: Vec<String>,
                link: String,
            }>,
        }>
    }
}

pub fn get_firmwares_vec(
    device_key: String,
    os_main_json_value: &Value,
    mut firmwares_vec: Vec<DeviceADBWebEntryOsEntry>,
    mut fw_key_vec: Vec<String>
) -> (Vec<DeviceADBWebEntryOsEntry>, Vec<String>) {
    let os_array = os_main_json_value.as_array().unwrap();

    for os_obj in os_array {
        let device_map = json::get_string_array(os_obj, "device_map");
        if !device_map.contains(&device_key) { continue }

        let mut sources: Vec<DeviceADBWebEntryOsEntrySource> = Vec::new();
        let source_data_array = os_obj["sources"].as_array().unwrap();
        
        for source_data in source_data_array {
            let source_type = json::get_string(source_data, "type");
            if source_type == *"ota" { continue }

            let device_map = json::get_string_array(source_data, "device_map");
            if !device_map.contains(&device_key) { continue };

            let link_array = source_data["links"].as_array().unwrap();
            let primary_link = &link_array[0];

            sources.push(DeviceADBWebEntryOsEntrySource {
                r#type: source_type,
                device_map,
                link: json::get_string(primary_link, "url")
            })
        }

        let entry = DeviceADBWebEntryOsEntry {
            os_str:   json::get_string(os_obj, "os_str"),
            version:  json::get_string(os_obj, "version"),
            build:    json::get_string(os_obj, "build"),
            key:      json::get_string(os_obj, "key"),
            released: json::get_string(os_obj, "released"),

            rc:       json::get_bool(os_obj, "rc"),
            beta:     json::get_bool(os_obj, "beta"),
            rsr:      json::get_bool(os_obj, "rsr"),
            internal: json::get_bool(os_obj, "internal"),

            preinstalled: json::get_string_array(os_obj, "preinstalled"),
            sources
        };

        if !fw_key_vec.contains(&entry.key) {
            fw_key_vec.push(entry.key.clone());
            firmwares_vec.push(entry);
        }
    }
    
    (
        firmwares_vec,
        fw_key_vec
    )
}

pub fn convert_device_entry_to_device_adb_web_entry(device_entry: device::DeviceEntry, os_main_json_value: &Value) -> DeviceADBWebEntry {
    let (firmwares, _fw_key_vec) = get_firmwares_vec(device_entry.key.clone(), os_main_json_value, Vec::new(), Vec::new());

    DeviceADBWebEntry {
        name: device_entry.name,
        key: device_entry.key,
        r#type: device_entry.r#type,
        identifier: device_entry.identifier,
        model: device_entry.model,
        board: device_entry.board,
        released: device_entry.released,
        soc: device_entry.soc,
        arch: vec![device_entry.arch],
        firmwares
    }
}