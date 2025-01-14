use axum::Json;
use serde::Serialize;
use serde_json::{from_reader, ser::PrettyFormatter, to_writer_pretty, Serializer, Value};
use std::{fs::File, io::BufReader};

#[allow(clippy::upper_case_acronyms)]
pub(crate) type JSON = Json<Value>;

pub fn read_json(path: &str) -> Value {
    let json_reader = BufReader::new(match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Path {} not found.", path),
    });
    match from_reader(json_reader) {
        Ok(value) => value,
        Err(_) => panic!("Unable to read JSON."),
    }
}

pub fn write_json<T: Serialize>(path: &str, value: T) {
    let file = File::create(path).unwrap();
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    match to_writer_pretty(file, &value) {
        Ok(_) => (),
        Err(_) => panic!("Unable to write JSON."),
    }
}

pub fn get_keys(value: &Value) -> Vec<String> {
    let mut keys = Vec::new();
    if let Value::Object(map) = value {
        for key in map.keys() {
            keys.push(key.to_string());
        }
    }
    keys
}

pub fn get_values(value: &Value) -> Vec<Value> {
    let mut values = Vec::new();
    if let Value::Object(map) = value {
        for value in map.values() {
            values.push(value.clone());
        }
    }
    values
}

pub fn get_map(value: &Value) -> Vec<(String, Value)> {
    let mut map = Vec::new();
    if let Value::Object(obj) = value {
        for (key, value) in obj.iter() {
            map.push((key.to_string(), value.clone()));
        }
    }
    map
}

pub fn get_length(value: &Value) -> usize {
    if let Value::Array(array) = value {
        array.len()
    } else {
        0
    }
}
