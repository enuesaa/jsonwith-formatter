use wasm_bindgen::prelude::*;

pub mod json;
pub mod yaml;

use crate::json::serializer::Serializer;
use crate::yaml::deserializer::Deserializer;

#[wasm_bindgen]
pub fn greet(value: &str) {
  let mut serializer = Serializer::new();
  serializer.serialize(&value);

  let values = serializer.values.clone();
  let mut deserializer = Deserializer::new(values);
  deserializer.print();
}
