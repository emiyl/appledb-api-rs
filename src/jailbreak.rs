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
    pub struct JailbreakEntry {
        name: String,
        key: String,
        alias: Vec<String>,
        priority: u64,
        hideFromGuide: bool,
        info: #[derive(FieldNamesAsArray)] struct JailbreakEntryInfo {
            website: UrlStruct,
            wiki: UrlStruct,
            guide: Vec<struct JailbreakEntryGuide {
                name: String,
                url: String,
                pkgman: String,
                updateLink: Vec<UrlStruct>,
                firmwares: Vec<String>,
                devices: Vec<String>
            }>,
            latestVer: String,
            color: String,
            icon: String,
            notes: String,
            jailbreaksmeapp: bool,
            r#type: String,
            firmwares: Vec<String>,
            soc: String
        },
        compatibility: Vec<struct JailbreakEntryCompatibility {
            firmwares: Vec<String>,
            devices: Vec<String>
        }>
    }
}


fn match_info(
    mut info: JailbreakEntryInfo,
    json: &Value,
    field: &str
) -> JailbreakEntryInfo {
    match field {
        "website" => {
            info.website = UrlStruct {
                name: json::get_string(&json[field], "name"),
                url: json::get_string(&json[field], "url")
            };
        }
        "wiki" => {
            info.wiki = UrlStruct {
                name: json::get_string(&json[field], "name"),
                url: json::get_string(&json[field], "url")
            }
        }
        "guide" => {
            fn get_update_link(
                json: &Value
            ) -> Vec<UrlStruct> {
                let link_list = json.as_array().unwrap();
                let mut link_vec: Vec<UrlStruct> = Vec::new();
                for link in link_list {
                    let new_link = UrlStruct {
                        name: json::get_string(link, "text"),
                        url: json::get_string(link, "link")
                    };
                    link_vec.push(new_link);
                }
                link_vec
            }

            fn match_guide(
                mut guide: JailbreakEntryGuide,
                json: &Value,
                field: &str
            ) -> JailbreakEntryGuide {
                match field {
                    "name" => guide.name = json::get_string(json, field),
                    "url" => guide.url = json::get_string(json, field),
                    "pkgman" => guide.pkgman = json::get_string(json, field),
                    "updateLink" => guide.updateLink = get_update_link(&json[field]),
                    "firmwares" => guide.firmwares = json::get_string_array(json, field),
                    "devices" => guide.devices = json::get_string_array(json, field),
                    _ => {}
                }
                guide
            }

            let guide_list = json[field].as_array().unwrap();
            let mut guide_vec: Vec<JailbreakEntryGuide> = Vec::new();
            for guide in guide_list {
                let mut new_guide: JailbreakEntryGuide = Default::default();
                let guide_field_list = json::get_object_field_list(guide);

                for guide_field in guide_field_list {
                    new_guide = match_guide(new_guide, guide, guide_field);
                }
                guide_vec.push(new_guide);
            }
            info.guide = guide_vec;
        },
        "latestVer" => info.latestVer = json::get_string(&json, field),
        "color" => info.color = json::get_string(&json, field),
        "icon" => info.icon = json::get_string(&json, field),
        "notes" => info.notes = json::get_string(&json, field),
        "jailbreaksmeapp" => info.jailbreaksmeapp = json::get_bool(&json, field),
        "r#type" => info.r#type = json::get_string(&json, "type"),
        "firmwares" => info.firmwares = json::get_string_array(&json, field),
        "soc" => info.soc = json::get_string(&json, field),
        _ => println!("WARNING: Unknown info field: {}", field),
    }

    info
}

fn match_entry(
    mut entry: JailbreakEntry,
    json: &Value,
    field: &str,
    field_exists_in_json: bool
) -> JailbreakEntry {
    match field {
        "name" => entry.name = json::get_string(json, field),
        "key" => {
            entry.key = if field_exists_in_json {
                json::get_string(json, field)
            } else {
                json::get_string(json, "name")
            }
        },
        "alias" => entry.alias = json::get_vec_from_string_or_string_vec(json, field),
        "priority" => entry.priority = json::get_u64(json, field),
        "hideFromGuide" => entry.hideFromGuide = json::get_bool(json, field),
        "info" => {
            let mut info_object: JailbreakEntryInfo = Default::default();
            let info_field_list = JailbreakEntryInfo::FIELD_NAMES_AS_ARRAY;

            let info_json = &json[field];
            let info_json_field_list = json::get_object_field_list(&info_json);
            
            for info_field in info_field_list {
                let info_field_exists_in_json = info_json_field_list.contains(&&info_field.to_string());
                if !info_field_exists_in_json { continue }

                info_object = match_info(info_object, &info_json, info_field);
            }

            entry.info = info_object;
        }
        "compatibility" => 'compat: {
            if !field_exists_in_json { break 'compat; }

            fn match_compat(
                mut compat: JailbreakEntryCompatibility,
                json: &Value,
                field: &str
            ) -> JailbreakEntryCompatibility {
                match field {
                    "firmwares" => compat.firmwares = json::get_string_array(json, field),
                    "devices" => compat.devices = json::get_string_array(json, field),
                    _ => {}
                }
                compat
            }

            fn get_compat(
                json: &Value
            ) -> JailbreakEntryCompatibility {
                let mut compat: JailbreakEntryCompatibility = Default::default();
                let field_list = json::get_object_field_list(json);
                for field in field_list {
                    compat = match_compat(compat, json, &field);
                }
                compat
            }
            
            let compat_list = json[field].as_array().unwrap();
            let mut compat_vec: Vec<JailbreakEntryCompatibility> = Vec::new();
            for compat in compat_list {
                compat_vec.push(get_compat(compat));
            }
            entry.compatibility = compat_vec;
        }
        _ => println!("Unknown key"),
    }
    entry
}

fn create_jailbreak_entry_from_json(json: &Value) -> JailbreakEntry {
    let mut entry: JailbreakEntry = Default::default();
    let field_list = json::get_object_field_list(json);

    for field in JailbreakEntry::FIELD_NAMES_AS_ARRAY {
        let field_exists_in_json = field_list.contains(&&field.to_string());
        entry = match_entry(entry, json, field, field_exists_in_json);
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
