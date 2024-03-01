use crate::file;
use crate::json;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use struct_field_names_as_array::FieldNamesAsArray;
use url::Url;

#[derive(Default, Serialize, Clone)]
#[allow(non_camel_case_types)]
enum OsEntryAppleDBWebImageAlign {
    #[default]
    left,
    right,
}

#[derive(Default, Serialize, Clone)]
struct OsEntryAppleDBWebImage {
    id: String,
    align: OsEntryAppleDBWebImageAlign,
}

#[derive(Default, Serialize, Clone)]
struct OsEntrySourceLink {
    url: String,
    active: bool,
}

#[derive(Default, Serialize, Clone)]
#[allow(non_snake_case)]
struct OsEntrySourceWindowsUpdateDetails {
    updateId: String,
    revisionId: String,
}

#[derive(Default, Serialize, Clone)]
#[allow(non_snake_case)]
struct OsEntrySource {
    r#type: String,
    prerequisiteBuild: Vec<String>,
    deviceMap: Vec<String>,
    osMap: Vec<String>,
    windowsUpdateDetails: OsEntrySourceWindowsUpdateDetails,
    links: Vec<OsEntrySourceLink>,
    hashes: BTreeMap<String, String>,
    skipUpdateLinks: bool,
    size: u64,
}

#[derive(Default, Serialize, FieldNamesAsArray, Clone)]
#[allow(non_snake_case)]
pub struct OsEntry {
    osStr: String,
    version: String,
    safariVersion: Vec<String>,
    build: String,
    uniqueBuild: String,
    pub key: String,
    embeddedOSBuild: String,
    bridgeOSBuild: String,
    buildTrain: String,
    released: String,
    rc: bool,
    beta: bool,
    rsr: bool,
    internal: bool,
    hideFromLatestVersions: bool,
    preinstalled: Vec<String>,
    notes: String,
    releaseNotes: String,
    securityNotes: String,
    ipd: BTreeMap<String, String>,
    appledbWebImage: OsEntryAppleDBWebImage,
    appledbWebUrl: String,
    pub appledbApiUrl: String,
    deviceMap: Vec<String>,
    osMap: Vec<String>,
    sources: Vec<OsEntrySource>,
}

pub fn create_os_entry_from_json(json: &Value) -> OsEntry {
    let mut entry: OsEntry = Default::default();
    let json_keys = json::get_object_keys(json);

    for key in OsEntry::FIELD_NAMES_AS_ARRAY {
        match key {
            "osStr" => entry.osStr = json::get_string(json, key),
            "version" => {
                if json_keys.contains(&&key.to_string()) {
                    entry.version = json::get_string(json, key)
                } else {
                    entry.version = json::get_string(json, "build")
                }
            }
            "safariVersion" => {
                if json[key].is_array() {
                    entry.safariVersion = json::get_string_array(json, key)
                } else if json[key].is_string() {
                    entry.safariVersion = vec![json::get_string(json, key)]
                }
            }
            "build" => {
                let build = json::get_string(json, key);
                if json_keys.contains(&&key.to_string()) {
                    entry.build = build;
                }
            }
            "uniqueBuild" => {
                let unique_build = json::get_string(json, key);
                if json_keys.contains(&&key.to_string()) {
                    // If uniqueBuild is defined in JSON, use JSON value
                    entry.uniqueBuild = unique_build;
                } else {
                    // Else, generate from build number
                    if json_keys.contains(&&"build".to_string()) {
                        entry.uniqueBuild = entry.build.clone()
                    } else {
                        entry.uniqueBuild = entry.version.clone()
                    }
                }
            }
            "key" => {
                let key = json::get_string(json, key);
                if json_keys.contains(&&key.to_string()) {
                    // If key is defined in JSON, use JSON value
                    entry.key = key;
                } else {
                    // Else, generate from osStr and build
                    let mut build = entry.build.clone();
                    if !json_keys.contains(&&"build".to_string()) {
                        build = json::get_string(json, "version");
                    }
                    entry.key = [&entry.osStr, ";", &build].concat();
                }
            }
            "embeddedOSBuild" => entry.embeddedOSBuild = json::get_string(json, key),
            "bridgeOSBuild" => entry.bridgeOSBuild = json::get_string(json, key),
            "buildTrain" => entry.buildTrain = json::get_string(json, key),
            "released" => {
                let released = json::get_string(json, key);
                if json_keys.contains(&&key.to_string()) {
                    // If released is defined in JSON, use JSON value
                    entry.released = released;
                } else {
                    // Else, default to 1970-01-01
                    entry.released = "1970-01-01".to_string();
                }
            }
            "rc" => entry.rc = json::get_bool(json, key),
            "beta" => entry.beta = json::get_bool(json, key),
            "rsr" => entry.rsr = json::get_bool(json, key),
            "internal" => entry.internal = json::get_bool(json, key),
            "hideFromLatestVersions" => entry.hideFromLatestVersions = json::get_bool(json, key),
            "preinstalled" => {
                if !json_keys.contains(&&key.to_string()) {
                    continue;
                    // If preinstalled does not exist in JSON, exit and leave the default value
                }
                let preinstalled = &json[key];
                // Preinstalled can be a bool or array
                if preinstalled.is_boolean() {
                    let preinstalled_bool = preinstalled.as_bool().unwrap();
                    // If preinstalled is true, use deviceMap as the preinstalled Array
                    // Else, leave as default
                    if preinstalled_bool {
                        entry.preinstalled = json::get_string_array(json, "deviceMap");
                    }
                } else if preinstalled.is_array() {
                    // If preinstalled is an array, use that value
                    entry.preinstalled = json::get_string_array(json, key);
                }
            }
            "notes" => entry.notes = json::get_string(json, key),
            "releaseNotes" => entry.releaseNotes = json::get_string(json, key),
            "securityNotes" => entry.securityNotes = json::get_string(json, key),
            "ipd" => {
                // Clones the ipd_key object in the JSON to a BTreeMap object
                let ipd_key_array = json::get_object_keys(&json[key]);
                let mut ipd_map: BTreeMap<String, String> = BTreeMap::new();
                for ipd_key in ipd_key_array {
                    ipd_map.insert(ipd_key.clone(), json::get_string(&json[key], ipd_key));
                }
                entry.ipd = ipd_map;
            }
            "appledbWebImage" => {
                if !json_keys.contains(&&key.to_string()) {
                    continue;
                }
                let align = json::get_string(&json[key], "align");

                fn get_align(align: String, entry: &OsEntry) -> OsEntryAppleDBWebImageAlign {
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
                            entry.osStr, entry.version, entry.build, align
                        );
                    OsEntryAppleDBWebImageAlign::left
                }

                let web_image = OsEntryAppleDBWebImage {
                    id: json::get_string(&json[key], "id"),
                    align: get_align(align, &entry),
                };
                entry.appledbWebImage = web_image;
            }
            "appledbWebUrl" => {
                // Create the URL
                let os_str = &entry.osStr;
                let unique_build = &entry.uniqueBuild;
                let paths = [os_str, "/", unique_build, ".html"]
                    .concat()
                    .replace(' ', "-");
                let url =
                    Url::parse("https://apiappledb.dev/firmware/").expect("Failed to parse URL");
                let url = url.join(&paths).expect("Failed to join URL");
                entry.appledbWebUrl = url.as_str().to_string();
            }
            "appledbApiUrl" => {
                // Create the URL
                let os_str = &entry.osStr;
                let unique_build = &entry.uniqueBuild;
                let paths = [os_str, "/", unique_build, ".json"]
                    .concat()
                    .replace(' ', "-");
                let url = Url::parse("https://api.appledb.dev/os/").expect("Failed to parse URL");
                let url = url.join(&paths).expect("Failed to join URL");
                entry.appledbApiUrl = url
                    .as_str()
                    .to_string()
                    .replace("https://api.appledb.dev", "");
            }
            "deviceMap" => entry.deviceMap = json::get_string_array(json, key),
            "osMap" => entry.osMap = json::get_string_array(json, key),
            "sources" => {
                if !json_keys.contains(&&key.to_string()) {
                    continue;
                }
                let source_array = json[key].as_array().unwrap();
                let mut source_vec: Vec<OsEntrySource> = Vec::new();
                for source in source_array {
                    // Create new OsEntrySource structs from JSON
                    let mut new_source: OsEntrySource = Default::default();
                    let source_key_array = json::get_object_keys(source);
                    for source_key in source_key_array {
                        match source_key.as_str() {
                            "type" => new_source.r#type = json::get_string(source, source_key),
                            "prerequisiteBuild" => {
                                if source[source_key].is_array() {
                                    new_source.prerequisiteBuild =
                                        json::get_string_array(source, source_key)
                                } else if source[source_key].is_string() {
                                    new_source.prerequisiteBuild =
                                        vec![json::get_string(source, source_key)]
                                }
                            }
                            "deviceMap" => {
                                new_source.deviceMap = json::get_string_array(source, source_key)
                            }
                            "osMap" => {
                                new_source.osMap = json::get_string_array(source, source_key)
                            }
                            "windowsUpdateDetails" => {
                                new_source.windowsUpdateDetails =
                                    OsEntrySourceWindowsUpdateDetails {
                                        updateId: json::get_string(&source[source_key], "updateId"),
                                        revisionId: json::get_string(
                                            &source[source_key],
                                            "revisionId",
                                        ),
                                    }
                            }
                            "links" => {
                                let link_array = source[source_key].as_array().unwrap();
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
                                let hash_object = &source[source_key];
                                let hash_key_array = json::get_object_keys(hash_object);
                                let mut hash_map: BTreeMap<String, String> = BTreeMap::new();
                                for hash_key in hash_key_array {
                                    hash_map.insert(
                                        hash_key.clone(),
                                        json::get_string(hash_object, hash_key),
                                    );
                                }
                                new_source.hashes = hash_map;
                            }
                            "skipUpdateLinks" => {
                                new_source.skipUpdateLinks = json::get_bool(source, source_key)
                            }
                            "size" => new_source.size = json::get_u64(source, source_key),
                            _ => {}
                        }
                    }
                    source_vec.push(new_source);
                }
                entry.sources = source_vec;
            }
            _ => println!("WARNING: Unknown key: {}", key),
        }
    }
    entry
}

pub fn get_os_entry_vec_from_path(file_path: &str) -> Vec<OsEntry> {
    let json_string = file::open_file_to_string(file_path);
    let mut json_vec = vec![json::parse_json(&json_string)];

    // Get duplicate entries
    if json_vec[0]["createDuplicateEntries"].is_array() {
        let create_duplicate_entries_json = json_vec[0]["createDuplicateEntries"].clone();
        let create_duplicate_entries_array = create_duplicate_entries_json.as_array().unwrap();
        let mut duplicate_entries_vec: Vec<Value> = Vec::new();
        for entry in create_duplicate_entries_array {
            let mut duplicate_json = json_vec[0].clone();
            let entry_keys = json::get_object_keys(entry);
            for key in entry_keys {
                duplicate_json[key] = entry[key].clone();
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
                sdk_mut["version"] = Value::String(json::get_string(sdk, "version") + "-SDK");
                sdk_mut["uniqueBuild"] = Value::String(json::get_string(sdk, "build") + "-SDK");
                sdk_mut["released"] = Value::String(json::get_string(sdk, "released"));
                let mut device_map_string = json::get_string(sdk, "osStr") + "-SDK";
                if device_map_string.contains("OS X") {
                    device_map_string = "macOS-SDK".to_string()
                }
                sdk_mut["deviceMap"] = Value::Array(vec![Value::String(device_map_string)]);
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
