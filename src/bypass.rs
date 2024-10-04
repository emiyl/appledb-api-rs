use crate::{json, OutputEntry, OutputFormat};
use serde::Serialize;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Default, Serialize, Clone)]
#[allow(non_snake_case)]
struct UrlStruct {
    name: String,
    url: String
}

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone)]]
    #[strikethrough[allow(non_snake_case)]]
    pub struct BypassEntry {
        name: String,
        bundleId: String,
        uri: String,
        icon: String,
        notes: String,
        bypasses: Vec<struct BypassEntryBypass {
            name: String,
            notes: String,
            version: String,
            guide: String,
            repository: struct BypassEntryBypassRepository {
                uri: String
            }
        }>
    }
}

fn match_entry(
    mut entry: BypassEntry,
    json: &Value,
    field: &str,
    field_exists_in_json: bool
) -> BypassEntry {
    match field {
        "name" => entry.name = json::get_string(json, field),
        "bundleId" => entry.bundleId = json::get_string(json, field),
        "uri" => entry.uri = json::get_string(json, field),
        "icon" => entry.icon = json::get_string(json, field),
        "notes" => entry.notes = json::get_string(json, field),
        _ => println!("Unknown key"),
    }
    entry
}

fn create_bypass_entry_from_json(json: &Value) -> BypassEntry {
    let mut entry: BypassEntry = Default::default();
    let field_list = json::get_object_field_list(json);

    for field in BypassEntry::FIELD_NAMES_AS_ARRAY {
        let field_exists_in_json = field_list.contains(&&field.to_string());
        entry = match_entry(entry, json, field, field_exists_in_json);
    }

    entry
}

pub fn process_entry(
    json_value: Value,
    value_vec: Vec<Value>,
) -> (Vec<OutputEntry>, OutputFormat) {
    let bypass_entry = create_bypass_entry_from_json(&json_value);
    
    (
        vec![OutputEntry {
            json: serde_json::to_string(&bypass_entry).expect("Failed to convert struct to JSON"),
            key: bypass_entry.bundleId.to_owned(),
        }],
        OutputFormat {
            value_vec,
            file_count: 0
        }
    )
}
