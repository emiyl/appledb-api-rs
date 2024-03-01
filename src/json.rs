use serde_json::Value;

pub fn parse_json(json_string: &str) -> Value {
    serde_json::from_str::<Value>(json_string).expect("JSON was not well-formatted")
}

pub fn get_object_keys(serde_value: &Value) -> Vec<&String> {
    let object = serde_value.as_object();
    if let Some(value) = object {
        value.keys().collect()
    } else {
        if !serde_value.is_null() {
            println!(
                "WARNING: Failed to get keys from serde_value {}",
                serde_value
            )
        }
        Vec::new()
    }
}

fn convert_serde_value_to_string(serde_value: &Value) -> String {
    if serde_value.is_string() {
        serde_value
            .as_str()
            .expect("Failed to convert serde_value to string")
            .to_string()
    } else {
        println!(
            "WARNING: Failed to convert serde_value {} to string, defaulting to empty string",
            serde_value
        );
        "".to_string()
    }
}

pub fn get_string_array(json: &Value, key: &str) -> Vec<String> {
    let json_keys = get_object_keys(json);
    if json_keys.contains(&&key.to_string()) {
        let array = json[key].as_array().unwrap();
        let mut vec_string = Vec::new();
        for i in array {
            vec_string.push(convert_serde_value_to_string(i));
        }
        vec_string
    } else {
        Vec::new()
    }
}

pub fn get_string(json: &Value, key: &str) -> String {
    let json_keys = get_object_keys(json);
    if json_keys.contains(&&key.to_string()) {
        convert_serde_value_to_string(&json[key])
    } else {
        "".to_string()
    }
}

pub fn get_bool(json: &Value, key: &str) -> bool {
    let json_keys = get_object_keys(json);
    if json_keys.contains(&&key.to_string()) {
        json[key].as_bool().unwrap()
    } else {
        false
    }
}

pub fn get_u64(json: &Value, key: &str) -> u64 {
    let json_keys = get_object_keys(json);
    if json_keys.contains(&&key.to_string()) && json[key].is_u64() {
        if json[key].is_u64() {
            return json[key].as_u64().unwrap();
        }
        println!(
            "WARNING: Failed to convert {} to u64, returning 0 instead",
            json[key]
        );
    }
    0
}
