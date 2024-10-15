use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;
use serde::Serialize;
use crate::common::{json, device};

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone)]]
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
        arch: String,
        firmwares: Vec<struct DeviceADBWebEntryOsEntry {
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
            sources: Vec<struct DeviceADBWebEntryOsEntrySource {
                r#type: String,
                device_map: Vec<String>,
                link: String,
            }>,
        }>
    }
}

pub fn convert_device_entry_to_device_adb_web_entry(device_entry: device::DeviceEntry, os_main_json_value: &Value) -> DeviceADBWebEntry {
    let mut firmwares: Vec<DeviceADBWebEntryOsEntry> = Vec::new();
    let os_array = os_main_json_value.as_array().unwrap();

    for os_obj in os_array {
        let device_map = json::get_string_array(&os_obj, "device_map");
        if !device_map.contains(&device_entry.key) { continue }

        let mut sources: Vec<DeviceADBWebEntryOsEntrySource> = Vec::new();
        let source_data_array = os_obj["sources"].as_array().unwrap();
        
        for source_data in source_data_array {
            let source_type = json::get_string(&source_data, "type");
            if source_type == "ota".to_string() { continue }

            let device_map = json::get_string_array(&source_data, "device_map");
            if !device_map.contains(&device_entry.key) { continue };

            let link_array = source_data["links"].as_array().unwrap();
            let primary_link = &link_array[0];

            sources.push(DeviceADBWebEntryOsEntrySource {
                r#type: source_type,
                link: json::get_string(&primary_link, "url")
            })
        }

        firmwares.push(DeviceADBWebEntryOsEntry {
            os_str:   json::get_string(&os_obj, "os_str"),
            version:  json::get_string(&os_obj, "version"),
            build:    json::get_string(&os_obj, "build"),
            key:      json::get_string(&os_obj, "key"),
            released: json::get_string(&os_obj, "released"),

            rc:       json::get_bool(&os_obj, "rc"),
            beta:     json::get_bool(&os_obj, "beta"),
            rsr:      json::get_bool(&os_obj, "rsr"),
            internal: json::get_bool(&os_obj, "internal"),

            preinstalled: json::get_string_array(&os_obj, "preinstalled"),
            sources:      sources
        })
    }

    DeviceADBWebEntry {
        name: device_entry.name,
        key: device_entry.key,
        r#type: device_entry.r#type,
        identifier: device_entry.identifier,
        model: device_entry.model,
        board: device_entry.board,
        released: device_entry.released,
        soc: device_entry.soc,
        arch: device_entry.arch,
        firmwares: firmwares
    }
}