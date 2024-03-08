use crate::{json, OutputEntry, OutputFormat};
use serde::Serialize;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Default, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct JailbreakEntry {
    name: String,
    priority: u64,
}

fn create_jailbreak_entry_from_json(json: &Value) -> JailbreakEntry {
    let mut entry: JailbreakEntry = Default::default();
    //let json_field_list = json::get_object_field_list(json);

    for field in JailbreakEntry::FIELD_NAMES_AS_ARRAY {
        match field {
            "name" => entry.name = json::get_string(json, field),
            "priority" => entry.priority = json::get_u64(json, field),
            _ => println!("Unknown key"),
        }
    }

    entry
}

pub fn process_entry(
    json_value: Value,
    value_vec: Vec<Value>,
) -> (Vec<OutputEntry>, OutputFormat) {
    let jailbreak_entry = create_jailbreak_entry_from_json(&json_value);
    
    (
        vec![OutputEntry {
            json: serde_json::to_string(&jailbreak_entry).expect("Failed to convert struct to JSON"),
            key: jailbreak_entry.name.to_owned(),
        }],
        OutputFormat {
            value_vec,
            file_count: 0
        }
    )
}
