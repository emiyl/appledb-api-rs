use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;
use serde::Serialize;
use crate::common::{json, device};
use chrono::{DateTime, Utc};

structstruck::strike! {
    #[strikethrough[derive(Default, Serialize, Clone)]]
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

#[derive(Default, Clone)]
pub struct DeviceADBWebEntryOsEntryWithTime {
    pub entry: DeviceADBWebEntryOsEntry,
    pub time: DateTime<Utc>
}

pub fn get_firmwares_vec(
    device_key: String,
    os_main_json_value: &Value,
    mut firmwares_vec: Vec<DeviceADBWebEntryOsEntryWithTime>,
    mut fw_key_vec: Vec<String>
) -> (Vec<DeviceADBWebEntryOsEntryWithTime>, Vec<String>) {
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

            let mut released_string = entry.released.clone();
            if released_string.is_empty() {
                released_string = "1970-01-01".to_string();
            } else if released_string.len() == 7 {
                released_string += "-01";
            } else if released_string.len() == 4 {
                released_string += "-01-01";
            }
            
            if released_string.len() < 11 {
                released_string += "T00:00:00-00:00";
            }

            firmwares_vec.push(DeviceADBWebEntryOsEntryWithTime {
                entry,
                time: DateTime::parse_from_rfc3339(released_string.as_str())
                    .unwrap()
                    .with_timezone(&Utc)
            });
        }
    }
    
    (
        firmwares_vec,
        fw_key_vec
    )
}

fn merge(left: Vec<DeviceADBWebEntryOsEntryWithTime>, right: Vec<DeviceADBWebEntryOsEntryWithTime>) -> Vec<DeviceADBWebEntryOsEntryWithTime> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
            if left[i].time <= right[j].time {
                result.push(left[i].clone());
                i += 1;
            } else {
                result.push(right[j].clone());
                j += 1;
            }
        }
        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);
        result
}

pub fn parallel_merge_sort(arr: Vec<DeviceADBWebEntryOsEntryWithTime>) -> Vec<DeviceADBWebEntryOsEntryWithTime> {
    if arr.len() <= 1 {
        return arr;
    }
    let middle = arr.len() / 2;
    let (left, right) = arr.split_at(middle);
    let left = left.to_vec(); // Clone the data
    let right = right.to_vec(); // Clone the data
    let (left, right) = rayon::join(|| parallel_merge_sort(left), || parallel_merge_sort(right));
    merge(left, right)
}

pub fn convert_device_entry_to_device_adb_web_entry(device_entry: device::DeviceEntry, os_main_json_value: &Value) -> DeviceADBWebEntry {
    let (firmwares_with_time, _fw_key_vec) = get_firmwares_vec(device_entry.key.clone(), os_main_json_value, Vec::new(), Vec::new());
    let firmwares = parallel_merge_sort(firmwares_with_time).into_iter().map(|fw| fw.entry).collect();

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