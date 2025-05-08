use serde_json::{json, Value};

pub fn param_map(pairs: &[(&str, Value)]) -> Vec<(String, Value)> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

pub fn map(json_str: &str) -> serde_json::Map<String, Value> {
    serde_json::from_str(json_str).expect("invalid JSON object")
}

pub fn example_input() -> Value {
    json!([
        { "category": "fruit" },
        { "category": "fruit" },
        { "category": "vegetable" }
    ])
}
