use serde_json::Value;

pub fn parse_json(json_string: &str) -> Value {
    serde_json::from_str::<Value>(json_string).expect("JSON was not well-formatted")
}

pub fn get_object_field_list(serde_value: &Value) -> Vec<&String> {
    let object = serde_value.as_object();
    if let Some(value) = object {
        value.keys().collect()
    } else {
        if !serde_value.is_null() {
            println!(
                "WARNING: Failed to get field list from serde_value {}",
                serde_value
            )
        }
        Vec::new()
    }
}

pub fn get_string_array(json: &Value, key: &str) -> Vec<String> {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&key.to_string()) {
        let array = json[key].as_array().unwrap();
        let mut vec_string = Vec::new();
        for i in array {
            vec_string.push(
                if i.is_string() {
                    i.as_str().unwrap()
                } else {
                    ""
                }
                .to_string(),
            )
        }
        vec_string
    } else {
        Vec::new()
    }
}

pub fn get_string(json: &Value, key: &str) -> String {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&key.to_string()) && json[key].is_string() {
        json[key].as_str().unwrap()
    } else {
        ""
    }
    .to_string()
}

pub fn get_bool(json: &Value, key: &str) -> bool {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&key.to_string()) && json[key].is_boolean() {
        json[key].as_bool().unwrap()
    } else {
        false
    }
}

pub fn get_u64(json: &Value, key: &str) -> u64 {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&key.to_string()) && json[key].is_u64() {
        json[key].as_u64().unwrap()
    } else {
        0
    }
}
