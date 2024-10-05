use struct_field_names_as_array::FieldNamesAsArray;
use serde::Serialize;
use std::collections::BTreeMap;
use crate::{file, json, os};

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone)]]
    pub struct OsADBWebEntry {
        pub os_str: String,
        version: String,
        build: String,
        pub key: String,
        released: String,
        rc: bool,
        beta: bool,
        rsr: bool,
        internal: bool,
        preinstalled: Vec<String>,
        appledb_web: os::OsEntryAppleDBWeb,
        device_map: Vec<struct OsADBWebEntryDevice {
            name: String,
            key: String,
            released: Vec<String>
        }>,
        sources: Vec<struct OsADBWebEntrySource {
            r#type: String,
            device_map: Vec<String>,
            links: Vec<os::OsEntrySourceLink>,
            hashes: BTreeMap<String, String>,
            size: u64,
        }>,
    }
}



pub fn convert_os_entry_to_os_adb_web_entry(os_entry: os::OsEntry) -> OsADBWebEntry {
    let mut sources: Vec<OsADBWebEntrySource> = Vec::new();
    for source in os_entry.sources {
        sources.push(OsADBWebEntrySource {
            r#type: source.r#type,
            device_map: source.device_map,
            links: source.links,
            hashes: source.hashes,
            size: source.size
        })
    }

    let mut device_map: Vec<OsADBWebEntryDevice> = Vec::new();
    for device in os_entry.device_map {
        let path = ["./out/device/key/", &device, ".json"].concat();
        let json_string = file::open_file_to_string(&path);
        let json_value = json::parse_json(&json_string);
        let name = json::get_string(&json_value, "name");
        device_map.push(OsADBWebEntryDevice {
            name: json::get_string(&json_value, "name"),
            key: json::get_string(&json_value, "key"),
            released: json::get_string_array(&json_value, "released")
        });
    }

    OsADBWebEntry {
        os_str: os_entry.os_str,
        version: os_entry.version,
        build: os_entry.build,
        key: os_entry.key,
        released: os_entry.released,
        rc: os_entry.rc,
        beta: os_entry.beta,
        rsr: os_entry.rsr,
        internal: os_entry.internal,
        preinstalled: os_entry.preinstalled,
        appledb_web: os_entry.appledb_web,
        device_map: device_map,
        sources: sources
    }
}