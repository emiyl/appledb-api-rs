use crate::json;
use serde::Serialize;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Default, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct DeviceGroup {
    name: String,
    pub key: String,
    r#type: String,
    devices: Vec<String>,
    hideChildren: bool,
    subgroups: Vec<DeviceGroup>,
}

pub fn create_device_group_from_json(json: &Value) -> DeviceGroup {
    let mut entry: DeviceGroup = Default::default();
    let json_field_list = json::get_object_field_list(json);

    for field in DeviceGroup::FIELD_NAMES_AS_ARRAY {
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
            "hideChildren" => entry.hideChildren = json::get_bool(json, field),
            "subgroups" => {
                let mut subgroup_vec: Vec<DeviceGroup> = Vec::new();
                if !json[field].is_array() {
                    entry.subgroups = subgroup_vec;
                    continue;
                }
                let subgroup_json_vec = json[field].as_array().unwrap();
                for subgroup_json in subgroup_json_vec {
                    subgroup_vec.push(create_device_group_from_json(subgroup_json))
                }
                entry.subgroups = subgroup_vec
            }
            _ => println!("Unknown key"),
        }
    }

    entry
}
