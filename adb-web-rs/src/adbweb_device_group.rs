use serde_json::{Map, Value};
use struct_field_names_as_array::FieldNamesAsArray;
use serde::Serialize;
use crate::common::{json, device_group};
use crate::adbweb_device;

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone)]]
    #[allow(non_snake_case)]
    pub struct DeviceGroupADBWebEntry {
        name: String,
        pub key: String,
        r#type: String,
        devices: Vec<String>,
        hide_children: bool,
        subgroups: Vec<DeviceGroupADBWebEntry>,
        identifier: Vec<String>,
        model: Vec<String>,
        board: Vec<String>,
        released: Vec<String>,
        soc: Vec<String>,
        arch: Vec<String>,
        firmwares: Vec<adbweb_device::DeviceADBWebEntryOsEntry>
    }
}

pub fn convert_device_group_entry_to_device_group_adb_web_entry(device_group_entry: device_group::DeviceGroupEntry, device_main_map_value: &Value) -> DeviceGroupADBWebEntry {
    let mut identifier: Vec<String> = Vec::new();
    let mut model: Vec<String> = Vec::new();
    let mut board: Vec<String> = Vec::new();
    let mut released: Vec<String> = Vec::new();
    let mut soc: Vec<String> = Vec::new();
    let mut arch: Vec<String> = Vec::new();
    let mut firmwares: Vec<adbweb_device::DeviceADBWebEntryOsEntry> = Vec::new();
    let mut fw_key_vec: Vec<String> = Vec::new();

    let device_key_iter = device_group_entry.devices.clone().into_iter();

    for device_key in device_key_iter {
        let device = device_main_map_value[device_key].as_object().unwrap();

        fn get_device_info_string(device: &Map<String, Value>, mut internal_vec: Vec<String>, field_str: &str) -> Vec<String> {
            let string_array = json::get_string_array(&serde_json::to_value(device).unwrap(), field_str);
            for iter in string_array.iter() {
                if !internal_vec.contains(iter) {
                    internal_vec.push(iter.to_string());
                }
            }
            internal_vec
        }

        identifier  = get_device_info_string(device, identifier, "identifier");
        model       = get_device_info_string(device, model, "model");
        released    = get_device_info_string(device, released, "released");
        board       = get_device_info_string(device, board, "board");
        soc         = get_device_info_string(device, soc, "soc");
        arch        = get_device_info_string(device, arch, "arch");

        (firmwares, fw_key_vec) = adbweb_device::get_firmwares_vec(
            device["key"].as_str().unwrap().to_string(),
            &device_main_map_value["os_main_json_value"],
            firmwares,
            fw_key_vec
        );    
    }

    DeviceGroupADBWebEntry {
        name: device_group_entry.name,
        key: device_group_entry.key,
        r#type: device_group_entry.r#type,
        devices: device_group_entry.devices,
        hide_children: device_group_entry.hide_children,
        subgroups: device_group_entry.subgroups.into_iter()
            .map(|sg| convert_device_group_entry_to_device_group_adb_web_entry(sg, device_main_map_value))
            .collect(),
        identifier,
        model,
        board,
        released,
        soc,
        arch,
        firmwares,
    }
}