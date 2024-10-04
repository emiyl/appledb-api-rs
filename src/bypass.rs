use crate::{file, json, OutputEntry, OutputFormat};
use serde::Serialize;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone, Debug)]]
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

fn grab_bypass_from_json(
    mut bypass: BypassEntryBypass
) -> BypassEntryBypass {
    let key = bypass.name.clone().to_ascii_lowercase().replace(' ', "-");
    let path = ["./appledb/bypassTweaks/", &key, ".json"].concat();
    let bypass_tweak_file_string = file::open_file_to_string(&path);
    let bypass_tweak_json_value = json::parse_json(&bypass_tweak_file_string);
    
    bypass.name = json::get_string(&bypass_tweak_json_value, "name");
    bypass.notes = json::get_string(&bypass_tweak_json_value, "notes");
    bypass.version = json::get_string(&bypass_tweak_json_value, "version");
    bypass.guide = json::get_string(&bypass_tweak_json_value, "guide");
    bypass.repository = BypassEntryBypassRepository {
        uri: json::get_string(&bypass_tweak_json_value["repository"], "uri")
    };
    
    bypass
}

fn get_bypasses(
    mut entry: BypassEntry,
    json: Value,
    field_exists_in_json: bool
) -> BypassEntry {
    if !field_exists_in_json || json.is_null() { return entry }

    let bypass_list = json.as_array().unwrap();
    let mut bypass_vec: Vec<BypassEntryBypass> = Vec::new();
    for bypass in bypass_list {
        let mut new_bypass: BypassEntryBypass = Default::default();

        new_bypass.name = json::get_string(bypass, "name");
        if new_bypass.name.len() < 1 { return entry }

        new_bypass = grab_bypass_from_json(new_bypass);
        new_bypass.notes = json::get_string(bypass, "notes");

        bypass_vec.push(new_bypass);
    }

    entry.bypasses = bypass_vec;
    entry
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
        "bypasses" => entry = get_bypasses(entry, json[field].clone(), field_exists_in_json),
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
