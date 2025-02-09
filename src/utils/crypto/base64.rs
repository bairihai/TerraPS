use std::fmt::Display;

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};

pub fn encode<T: Display>(input: T) -> String {
    let mut output_buf = String::new();
    STANDARD.encode_string(input.to_string(), &mut output_buf);
    output_buf
}

pub fn encode_bytes(input: Vec<u8>) -> String {
    STANDARD.encode(input)
}

#[allow(dead_code)]
pub fn decode<T: Display>(input: T) -> Result<String> {
    Ok(String::from_utf8(STANDARD.decode(input.to_string())?)?)
}

pub fn decode_bytes<T: Display>(input: T) -> Result<Vec<u8>> {
    Ok(STANDARD.decode(input.to_string())?)
}
