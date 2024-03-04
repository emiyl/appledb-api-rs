use serde_json::{json, Value};

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

pub fn get_vec_from_string_or_string_vec(json: &Value, field: &str) -> Vec<String> {
    if json[field].is_string() {
        vec![json[field].as_str().unwrap().to_string()]
    } else if json[field].is_array() {
        get_string_array(json, field)
    } else {
        vec![]
    }
}

pub fn get_string_array(json: &Value, field: &str) -> Vec<String> {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&field.to_string()) {
        let array = json[field].as_array().unwrap();
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

pub fn get_string(json: &Value, field: &str) -> String {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&field.to_string()) && json[field].is_string() {
        json[field].as_str().unwrap()
    } else {
        ""
    }
    .to_string()
}

pub fn get_bool(json: &Value, field: &str) -> bool {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&field.to_string()) && json[field].is_boolean() {
        json[field].as_bool().unwrap()
    } else {
        false
    }
}

pub fn get_u64(json: &Value, field: &str) -> u64 {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&field.to_string()) && json[field].is_u64() {
        json[field].as_u64().unwrap()
    } else {
        0
    }
}

pub fn get_object(json: &Value, field: &str) -> Value {
    let json_field_list = get_object_field_list(json);
    if json_field_list.contains(&&field.to_string()) && json[field].is_object() {
        json[field].clone()
    } else {
        json!("{}")
    }
}
