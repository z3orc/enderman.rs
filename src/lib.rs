use serde_json::{Value};

pub fn fetch_json(url: &str) -> Value{
    let response = reqwest::blocking::get(url).unwrap();
    let raw_json = response.text().unwrap();
    let raw_raw_json: Value = serde_json::from_str(&raw_json).expect("Could not parse json");
    return raw_raw_json
}