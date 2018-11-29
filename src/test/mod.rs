use std::vec::Vec;
use std::collections::BTreeMap;
use std::convert::From;
use crate::Value;
use crate::Array;
use crate::Object;
use crate::Null;
use crate::parse;

enum JsonValue {
    Null,
    Number(f64),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>)
}

struct JsonArray(Vec<JsonValue>);
struct JsonObject(BTreeMap<String, JsonValue>);

impl Array<JsonValue, JsonObject, JsonValue> for JsonArray {
    fn new() -> Self {
        JsonArray(Vec::new())
    }
    fn push(&mut self, v: JsonValue) {
        self.0.push(v)
    }
}

impl Object<JsonValue, JsonArray, JsonValue> for JsonObject {
    fn new<'b>() -> Self {
        JsonObject(BTreeMap::new())
    }
    fn insert(&mut self, k: String, v: JsonValue) {
        self.0.insert(k, v);
    }
}

impl Null<JsonValue, JsonArray, JsonObject> for JsonValue {
    fn new() -> Self {
        JsonValue::Null
    }
}
impl Value<JsonArray, JsonObject, JsonValue> for JsonValue {}

impl From<f64> for JsonValue {
    fn from(v: f64) -> Self {
        JsonValue::Number(v)
    }
}
impl From<bool> for JsonValue {
    fn from(v: bool) -> Self {
        JsonValue::Bool(v)
    }
}
impl From<String> for JsonValue {
    fn from(v: String) -> Self{
        JsonValue::String(v)
    }
}
impl From<JsonArray> for JsonValue {
    fn from(v: JsonArray) -> Self {
        JsonValue::Array(v.0)
    }
}
impl From<JsonObject> for JsonValue {
    fn from(v: JsonObject) -> Self {
        JsonValue::Object(v.0)
    }
}

impl std::fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            JsonValue::Null => f.write_str("null"),
            JsonValue::String(ref v) => f.write_fmt(format_args!("\"{}\"", v)),
            JsonValue::Number(ref v) => f.write_fmt(format_args!("{}", v)),
            JsonValue::Bool(ref v) => f.write_fmt(format_args!("{}", v)),
            JsonValue::Array(ref v) => f.write_fmt(format_args!("{:?}", v)),
            JsonValue::Object(ref v) => f.write_fmt(format_args!("{:#?}", v))
        }
    }
}

impl std::fmt::Display for JsonValue {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", *self))
    }
}

#[test]
fn test() {
    let data = include_str!("./test.json");
    let data_array: Vec<char> = data.chars().collect();
    let mut index = 0;
    let interpreted = parse::<JsonValue, JsonArray, JsonObject, JsonValue>(&*data_array, &mut index);
    assert_eq!(index, data_array.len() - 1);
    assert!(interpreted.is_some());
    // That means the parser has reached and the data is there.
    // We should test whether the data is good or not, but it is...boring.
    println!("{}", interpreted.unwrap()); // run with --nocapture to check result.
}

