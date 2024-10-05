use crate::{file, json, OutputEntry, OutputFormat, adb_web};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::{fs, io::Write, os::unix::fs::FileExt};
use struct_field_names_as_array::FieldNamesAsArray;
use url::Url;

structstruck::strike! {
    #[derive(FieldNamesAsArray)]
    #[strikethrough[derive(Default, Serialize, Clone)]]
    #[strikethrough[allow(non_snake_case)]]
    pub struct OsEntry {
        pub os_str: String,
        pub version: String,
        safari_version: Vec<String>,
        pub build: String,
        pub key: String,
        embeddedOS_build: String,
        bridgeOS_build: String,
        build_train: String,
        pub released: String,
        pub rc: bool,
        pub beta: bool,
        pub rsr: bool,
        pub internal: bool,
        pub preinstalled: Vec<String>,
        notes: String,
        release_notes: String,
        security_notes: String,
        ipd: BTreeMap<String, String>,
        pub appledb_web: #[derive(FieldNamesAsArray)] pub struct OsEntryAppleDBWeb {
            web_image: struct OsEntryAppleDBWebImage {
                id: String,
                align: #[allow(non_camel_case_types)]
                enum OsEntryAppleDBWebImageAlign {
                    #[default]
                    left,
                    right,
                },
            },
            web_url: String,
            api_url: String,
            hide_from_latest_versions: bool
        },
        pub device_map: Vec<String>,
        os_map: Vec<String>,
        pub sources: Vec<pub struct OsEntrySource {
            pub r#type: String,
            prerequisite_build: Vec<String>,
            pub device_map: Vec<String>,
            os_map: Vec<String>,
            windows_update_details: struct OsEntrySourceWindowsUpdateDetails {
                update_id: String,
                revision_id: String,
            },
            pub links: Vec<pub struct OsEntrySourceLink {
                url: String,
                active: bool,
            }>,
            pub hashes: BTreeMap<String, String>,
            skip_update_links: bool,
            pub size: u64,
        }>,
    }
}

fn create_os_entry_from_json(json: &Value) -> OsEntry {
    let mut entry: OsEntry = Default::default();
    let json_field_list = json::get_object_field_list(json);

    for field in OsEntry::FIELD_NAMES_AS_ARRAY {
        match field {
            "os_str" => entry.os_str = json::get_string(json, "osStr"),
            "version" => entry.version = json::get_string(json, field),
            "safari_version" => {
                if json[field].is_array() {
                    entry.safari_version = json::get_string_array(json, "safariVersion")
                } else if json[field].is_string() {
                    entry.safari_version = vec![json::get_string(json, "safariVersion")]
                }
            }
            "build" => entry.build = json::get_string(json, field),
            "key" => {
                if json_field_list.contains(&&field.to_string()) {
                    // If key is defined in JSON, use JSON value
                    let mut entry_key = json::get_string(json, field);
                    if !entry_key.starts_with(&entry.os_str) && !entry_key.ends_with('!') {
                        // If key doesn't start with os_str, and doesn't end with "!", then add os_str to the start
                        entry_key = [entry.os_str.clone(), ';'.to_string(), entry_key].concat()
                    } else if entry_key.ends_with('!') {
                        entry_key.pop();
                    }
                    entry.key = entry_key;
                } else {
                    // Else, generate from os_str and uniqueBuild/build/version
                    let key_second_part = if json_field_list.contains(&&"uniqueBuild".to_string()) {
                        json::get_string(json, "uniqueBuild")
                    } else if json_field_list.contains(&&"build".to_string()) {
                        json::get_string(json, "build")
                    } else {
                        json::get_string(json, "version")
                    };
                    entry.key = [&entry.os_str, ";", &key_second_part].concat();
                }
            }
            "embeddedOS_build" => entry.embeddedOS_build = json::get_string(json, "embeddedOSBuild"),
            "bridgeOS_build" => entry.bridgeOS_build = json::get_string(json, "bridgeOSBuild"),
            "build_train" => entry.build_train = json::get_string(json, "buildTrain"),
            "released" => {
                let released = json::get_string(json, field);
                if json_field_list.contains(&&field.to_string()) {
                    // If released is defined in JSON, use JSON value
                    entry.released = released;
                } else {
                    // Else, default to 1970-01-01
                    entry.released = "1970-01-01".to_string();
                }
            }
            "rc" => entry.rc = json::get_bool(json, field),
            "beta" => entry.beta = json::get_bool(json, field),
            "rsr" => entry.rsr = json::get_bool(json, field),
            "internal" => entry.internal = json::get_bool(json, field),
            "preinstalled" => {
                if !json_field_list.contains(&&field.to_string()) {
                    continue;
                    // If preinstalled does not exist in JSON, exit and leave the default value
                }
                let preinstalled = &json[field];
                // Preinstalled can be a bool or array
                if preinstalled.is_boolean() {
                    let preinstalled_bool = preinstalled.as_bool().unwrap();
                    // If preinstalled is true, use device_map as the preinstalled Array
                    // Else, leave as default
                    if preinstalled_bool {
                        entry.preinstalled = json::get_string_array(json, "deviceMap");
                    }
                } else if preinstalled.is_array() {
                    // If preinstalled is an array, use that value
                    entry.preinstalled = json::get_string_array(json, field);
                }
            }
            "notes" => entry.notes = json::get_string(json, field),
            "release_notes" => entry.release_notes = json::get_string(json, "releaseNotes"),
            "security_notes" => entry.security_notes = json::get_string(json, "securityNotes"),
            "ipd" => {
                // Clones the ipd_field object in the JSON to a BTreeMap object
                let ipd_field_list = json::get_object_field_list(&json[field]);
                let mut ipd_map: BTreeMap<String, String> = BTreeMap::new();
                for ipd_field in ipd_field_list {
                    ipd_map.insert(ipd_field.clone(), json::get_string(&json[field], ipd_field));
                }
                entry.ipd = ipd_map;
            }
            "appledb_web" => {
                let mut appledb_web_object: OsEntryAppleDBWeb = Default::default();
                let appledb_web_field_list = OsEntryAppleDBWeb::FIELD_NAMES_AS_ARRAY;

                for appledb_web_field in appledb_web_field_list {
                    match appledb_web_field {
                        "web_image" => {
                            if !json_field_list.contains(&&"appledbWebImage".to_string()) {
                                continue;
                            }

                            let align = json::get_string(&json["appledbWebImage"], "align");

                            fn get_align(
                                align: String,
                                entry: &OsEntry,
                            ) -> OsEntryAppleDBWebImageAlign {
                                // Value should only ever be left or right, so enum is used
                                if align == "left" {
                                    return OsEntryAppleDBWebImageAlign::left;
                                }
                                if align == "right" {
                                    return OsEntryAppleDBWebImageAlign::right;
                                }
                                // Default to aligning the image as left, issue warning if JSON is not "left" or "right"
                                println!(
                                        "WARNING: {} {} ({}) has unknown appledbWebImage alignment: '{}'. Defaulting to 'left'.",
                                        entry.os_str, entry.version, entry.build, align
                                    );
                                OsEntryAppleDBWebImageAlign::left
                            }

                            appledb_web_object.web_image = OsEntryAppleDBWebImage {
                                id: json::get_string(&json["appledbWebImage"], "id"),
                                align: get_align(align, &entry),
                            };
                        }
                        "web_url" => {
                            let paths = [entry.key.replace(';', "/"), ".html".to_string()]
                                .concat()
                                .replace(' ', "-");
                            let url = Url::parse("https://appledb.dev/firmware/")
                                .expect("Failed to parse URL");
                            let url = url.join(&paths).expect("Failed to join URL");
                            appledb_web_object.web_url = url.as_str().to_string();
                        }
                        "api_url" => {
                            let paths =
                                [entry.key.clone().replace(';', "/"), ".json".to_string()].concat();
                            let url = Url::parse("https://api.emiyl.com/firmware/")
                                .expect("Failed to parse URL");
                            let url = url.join(&paths).expect("Failed to join URL");
                            appledb_web_object.api_url = url.as_str().to_string();
                        }
                        "hide_from_latest_versions" => {
                            appledb_web_object.hide_from_latest_versions =
                                json::get_bool(json, "hideFromLatestVersions")
                        }
                        _ => println!("WARNING: Unknown AppleDB field: {}", appledb_web_field),
                    }
                }

                entry.appledb_web = appledb_web_object;
            }
            "device_map" => entry.device_map = json::get_string_array(json, "deviceMap"),
            "os_map" => entry.os_map = json::get_string_array(json, "osMap"),
            "sources" => {
                if !json_field_list.contains(&&field.to_string()) {
                    continue;
                }
                let source_array = json[field].as_array().unwrap();
                let mut source_vec: Vec<OsEntrySource> = Vec::new();
                for source in source_array {
                    // Create new OsEntrySource structs from JSON
                    let mut new_source: OsEntrySource = Default::default();
                    let source_field_list = json::get_object_field_list(source);
                    for source_field in source_field_list {
                        match source_field.as_str() {
                            "type" => new_source.r#type = json::get_string(source, source_field),
                            "prerequisite_build" => {
                                if source[source_field].is_array() {
                                    new_source.prerequisite_build =
                                        json::get_string_array(source, "prerequisiteBuild")
                                } else if source[source_field].is_string() {
                                    new_source.prerequisite_build =
                                        vec![json::get_string(source, "prerequisiteBuild")]
                                }
                            }
                            "device_map" => {
                                new_source.device_map = json::get_string_array(source, "deviceMap")
                            }
                            "os_map" => {
                                new_source.os_map = json::get_string_array(source, "osMap")
                            }
                            "windows_update_details" => {
                                new_source.windows_update_details =
                                    OsEntrySourceWindowsUpdateDetails {
                                        update_id: json::get_string(
                                            &source["windowsUpdateDetails"],
                                            "updateId",
                                        ),
                                        revision_id: json::get_string(
                                            &source["windowsUpdateDetails"],
                                            "revisionId",
                                        ),
                                    }
                            }
                            "links" => {
                                let link_array = source[source_field].as_array().unwrap();
                                let mut link_vec: Vec<OsEntrySourceLink> = Vec::new();
                                for link in link_array {
                                    let new_link = OsEntrySourceLink {
                                        url: json::get_string(link, "url"),
                                        active: json::get_bool(link, "active"),
                                    };
                                    link_vec.push(new_link);
                                }
                                new_source.links = link_vec;
                            }
                            "hashes" => {
                                let hash_object = &source[source_field];
                                let hash_field_list = json::get_object_field_list(hash_object);
                                let mut hash_map: BTreeMap<String, String> = BTreeMap::new();
                                for hash_field in hash_field_list {
                                    hash_map.insert(
                                        hash_field.clone(),
                                        json::get_string(hash_object, hash_field),
                                    );
                                }
                                new_source.hashes = hash_map;
                            }
                            "skip_update_links" => {
                                new_source.skip_update_links = json::get_bool(source, "skipUpdateLinks")
                            }
                            "size" => new_source.size = json::get_u64(source, source_field),
                            _ => {}
                        }
                    }
                    source_vec.push(new_source);
                }
                entry.sources = source_vec;
            }
            _ => println!("WARNING: Unknown field: {}", field),
        }
    }
    entry
}

fn get_os_entry_vec_from_path(json_value: Value) -> Vec<OsEntry> {
    let mut json_vec = vec![json_value];

    // Get duplicate entries
    if json_vec[0]["createDuplicateEntries"].is_array() {
        let create_duplicate_entries_json = json_vec[0]["createDuplicateEntries"].clone();
        let create_duplicate_entries_array = create_duplicate_entries_json.as_array().unwrap();
        let mut duplicate_entries_vec: Vec<Value> = Vec::new();
        for entry in create_duplicate_entries_array {
            let mut duplicate_json = json_vec[0].clone();
            let entry_field_list = json::get_object_field_list(entry);
            for field in entry_field_list {
                duplicate_json[field] = entry[field].clone();
            }
            duplicate_entries_vec.push(duplicate_json);
        }
        json_vec.append(&mut duplicate_entries_vec);
    }

    // Get SDK entries
    for json in json_vec.clone() {
        if json["sdks"].is_array() {
            let mut sdk_entries_vec: Vec<Value> = Vec::new();
            for sdk in json["sdks"].as_array().unwrap() {
                if !sdk.is_object() {
                    continue;
                }

                let mut sdk_mut = sdk.clone();
                sdk_mut["version"] = Value::String(json::get_string(sdk, "version") + " SDK");
                sdk_mut["build"] = Value::String(json::get_string(sdk, "build"));

                let sdk_field_list = json::get_object_field_list(sdk);
                let key = if sdk_field_list.contains(&&"key".to_string()) {
                    json::get_string(sdk, "key")
                } else if sdk_field_list.contains(&&"uniqueBuild".to_string()) {
                    json::get_string(sdk, "uniqueBuild")
                } else if sdk_field_list.contains(&&"build".to_string()) {
                    json::get_string(sdk, "build")
                } else {
                    json::get_string(sdk, "version")
                };
                sdk_mut["key"] = Value::String(key + "-SDK");

                sdk_mut["released"] = Value::String(json::get_string(sdk, "released"));
                let mut device_map_string = json::get_string(sdk, "osStr") + " SDK";
                if device_map_string.contains("OS X") {
                    device_map_string = "macOS SDK".to_string()
                }

                sdk_mut["device_map"] = Value::Array(vec![Value::String(device_map_string)]);
                sdk_mut["sdk"] = Value::Bool(true);

                sdk_entries_vec.push(sdk_mut);
            }
            json_vec.append(&mut sdk_entries_vec);
        }
    }

    let mut entry_vec: Vec<OsEntry> = Vec::new();
    for json in json_vec {
        entry_vec.push(create_os_entry_from_json(&json));
    }

    entry_vec
}

fn write_os_str_main_index_json_files(
    output_dir: &String,
    os_str: String,
    out_json: &String,
    key: &String,
    create_new_file: bool,
) -> u32 {
    let mut file_count: u32 = 0;
    let os_str_main_index_json_file_path =
        ["/main.json", "/index.json"].map(|str| [output_dir, &os_str, str].concat());
    for (i, file_path) in os_str_main_index_json_file_path.iter().enumerate() {
        let file_exists = file::path_exists(file_path);

        if !file_exists || create_new_file {
            file::write_string_to_file(file_path, &"[".to_string())
                .expect("Failed to write to os_str main/index json file");
            file_count += 1;
        }

        let mut json_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .unwrap();

        let buf = if i == 0 {
            [out_json, ","].concat()
        } else {
            ["\"", key, "\","].concat()
        };

        json_file
            .write_all(buf.as_bytes())
            .expect("Failed to write to os_str main/index json file");
    }

    file_count
}

pub fn finalise_entry(output_dir: &String, output: OutputFormat) -> OutputFormat {
    for output in &output.value_vec {
        let mut os_str = "";
        if output.is_string() {
            os_str = output.as_str().unwrap();
        }

        let main_index_json_list =
            ["/main.json", "/index.json"].map(|str| [output_dir, os_str, str].concat());
        for path in main_index_json_list {
            if file::open_file_to_string(path.as_str()).ends_with(',') {
                let file = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .unwrap();
                let len = file.metadata().unwrap().len();
                let offset = if len > 1 { len - 1 } else { len };
                file.write_at("]\n".as_bytes(), offset)
                    .expect("Failed to write to os_str main/index json file");
            };
        }
    }

    output
}

pub fn process_entry(
    json_value: Value,
    mut value_vec: Vec<Value>,
    output_dir: &String,
    adb_web: bool
) -> (Vec<OutputEntry>, OutputFormat) {
    let mut file_count: u32 = 0;
    let os_entry_vec = get_os_entry_vec_from_path(json_value);

    let mut output_entry_vec: Vec<OutputEntry> = Vec::new();
    for os_entry in os_entry_vec {
        let json = if adb_web {
            serde_json::to_string(&adb_web::convert_os_entry_to_os_adb_web_entry(os_entry.clone()))
        } else {
            serde_json::to_string(&os_entry)
        }.expect("Failed to convert struct to JSON");

        let output_entry = OutputEntry {
            json: json,
            key: os_entry.key.replace(';', "/"),
        };

        // OsEntry needs the firmware/<os_str>/<"main"|"index">.json files
        // Use os_str_vec to keep track of which os_str files have been created
        // Since the script appends to files, we need to know which files have already been created or not
        let os_str = os_entry.os_str;
        let os_str_value = Value::String(os_str.to_owned());
        let os_str_vec_contains = value_vec.contains(&os_str_value);
        if !os_str_vec_contains {
            value_vec.push(os_str_value)
        };

        file_count += write_os_str_main_index_json_files(
            output_dir,
            os_str,
            &output_entry.json,
            &output_entry.key,
            !os_str_vec_contains,
        );

        output_entry_vec.push(output_entry);
    }

    (output_entry_vec, OutputFormat { value_vec, file_count })
}
