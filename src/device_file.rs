use crate::json;
use serde::Serialize;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Default, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct DeviceEntry {
    name: String,
    pub key: String,
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

pub fn create_device_entry_from_json(json: &Value) -> DeviceEntry {
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
