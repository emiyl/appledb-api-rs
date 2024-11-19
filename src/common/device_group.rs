use crate::common::{json, write_entry, OutputEntry, OutputFormat};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Default, Serialize, Clone, Debug)]
pub struct DeviceGroupEntry {
    pub name: String,
    pub key: String,
    pub r#type: String,
    pub devices: Vec<String>,
    pub hide_children: bool,
    pub subgroups: Vec<DeviceGroupEntry>,
}

pub fn create_device_group_entry_from_json(json: &Value) -> DeviceGroupEntry {
    let mut entry: DeviceGroupEntry = Default::default();
    let json_field_list = json::get_object_field_list(json);

    for field in DeviceGroupEntry::FIELD_NAMES_AS_ARRAY {
        match field {
            "name" => entry.name = json::get_string(json, field),
            "key" => {
                entry.key = if json_field_list.contains(&&"key".to_string()) {
                    json::get_string(json, field)
                } else {
                    json::get_string(json, "name")
                }
            }
            "r#type" => entry.r#type = json::get_string(json, "type"),
            "devices" => entry.devices = json::get_string_array(json, field),
            "hide_children" => entry.hide_children = json::get_bool(json, "hideChildren"),
            "subgroups" => {
                let mut subgroup_vec: Vec<DeviceGroupEntry> = Vec::new();
                if !json[field].is_array() {
                    entry.subgroups = subgroup_vec;
                    continue;
                }
                let subgroup_json_vec = json[field].as_array().unwrap();
                for subgroup_json in subgroup_json_vec {
                    subgroup_vec.push(create_device_group_entry_from_json(subgroup_json))
                }
                entry.subgroups = subgroup_vec
            }
            _ => println!("Unknown key"),
        }
    }

    entry
}

pub fn process_entry(
    json_value: Value,
    mut value_vec: Vec<Value>,
    extra_input_value: &Value
) -> (Vec<OutputEntry>, OutputFormat) {
    let device_group = create_device_group_entry_from_json(&json_value);

    let device_group_devices = &device_group.devices;
    for device in device_group_devices {
        let value = Value::String(device.to_owned());
        if !value_vec.contains(&value) {
            value_vec.push(value);
        }
    }
    
    #[cfg(feature = "api")]
    let json = serde_json::to_string(&device_group).expect("Failed to convert struct to JSON");
    
    #[cfg(feature = "adb_web")]
    let json = serde_json::to_string(&crate::adbweb_device_group::convert_device_group_entry_to_device_group_adb_web_entry(device_group.clone(), extra_input_value)).expect("Failed to convert struct to JSON");

    (
        vec![OutputEntry {
            json,
            key: device_group.key.to_owned(),
        }],
        OutputFormat {
            value_vec,
            file_count: 0,
        }
    )
}

pub fn finalise_entry(
    output_dir: &String,
    all_devices_vec: &Vec<Value>,
    output: OutputFormat,
    main_index_json_file_array: &[fs::File; 2],
) -> OutputFormat {
    let devices_in_device_groups_vec = output.value_vec;
    let mut devices_not_in_device_groups_vec: Vec<&Value> = Vec::new();
    for device_obj in all_devices_vec {
        let device_key = &device_obj["key"];
        if !devices_in_device_groups_vec.contains(device_key) {
            devices_not_in_device_groups_vec.push(device_obj)
        }
    }

    let mut output = OutputFormat {
        value_vec: devices_in_device_groups_vec.to_owned(),
        file_count: 0,
    };
    for device in devices_not_in_device_groups_vec {
        output = write_entry(
            &crate::common::EntryType::DeviceGroup,
            device.to_owned(),
            output,
            output_dir,
            main_index_json_file_array,
            &json!("[]")
        );
    }

    output
}
