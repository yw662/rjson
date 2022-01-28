extern crate rjson;
use rjson::parse;
use rjson::Array;
use rjson::Null;
use rjson::Object;
use rjson::Value;
use std::collections::BTreeMap;
use std::convert::From;
use std::vec::Vec;

#[derive(PartialEq)]
pub enum JsonValue {
    Null,
    Number(f64),
    U64(u64),
    I64(i64),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
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
impl From<u64> for JsonValue {
    fn from(v: u64) -> Self {
        JsonValue::U64(v)
    }
}
impl From<i64> for JsonValue {
    fn from(v: i64) -> Self {
        JsonValue::I64(v)
    }
}
impl From<bool> for JsonValue {
    fn from(v: bool) -> Self {
        JsonValue::Bool(v)
    }
}
impl From<String> for JsonValue {
    fn from(v: String) -> Self {
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
            JsonValue::U64(ref v) => f.write_fmt(format_args!("{}", v)),
            JsonValue::I64(ref v) => f.write_fmt(format_args!("{}", v)),
            JsonValue::Bool(ref v) => f.write_fmt(format_args!("{}", v)),
            JsonValue::Array(ref v) => f.write_fmt(format_args!("{:?}", v)),
            JsonValue::Object(ref v) => f.write_fmt(format_args!("{:#?}", v)),
        }
    }
}

impl std::fmt::Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", *self))
    }
}

pub fn parse_json(data: &str) -> Option<JsonValue> {
    let data_array: Vec<char> = data.chars().collect();
    let mut index = 0;
    rjson::parse::<JsonValue, JsonArray, JsonObject, JsonValue>(&*data_array, &mut index)
}

impl JsonValue {
    pub fn f64(&self, key: &str) -> Option<f64> {
        match self {
            JsonValue::Object(o) => match o.get(key)? {
                JsonValue::Number(n) => Some(*n),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn u64(&self, key: &str) -> Option<u64> {
        match self {
            JsonValue::Object(o) => match o.get(key)? {
                JsonValue::U64(n) => Some(*n),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn i64(&self, key: &str) -> Option<i64> {
        match self {
            JsonValue::Object(o) => match o.get(key)? {
                JsonValue::I64(n) => Some(*n),
                _ => None,
            },
            _ => None,
        }
    }
}

#[test]
fn test() {
    let data = include_str!("./test.json");
    let data_array: Vec<char> = data.chars().collect();
    let mut index = 0;
    let interpreted =
        parse::<JsonValue, JsonArray, JsonObject, JsonValue>(&*data_array, &mut index);
    assert_eq!(index, data_array.len() - 1);
    assert!(interpreted.is_some());
    // That means the parser has reached and the data is there.
    // We should test whether the data is good or not, but it is...boring.
    println!("{}", interpreted.unwrap()); // run with --nocapture to check result.
}

#[cfg(feature = "integer")]
#[test]
fn test_f64() {
    let json_val = parse_json(r#"{"number": 1.23}"#).unwrap();
    let val = json_val.f64("number").unwrap();
    assert_eq!(val, 1.23);

    let json_val = parse_json(r#"{"number": -1.23}"#).unwrap();
    let val = json_val.f64("number").unwrap();
    assert_eq!(val, -1.23);

    let json_val = parse_json(r#"{"number": 123}"#).unwrap();
    let val = json_val.f64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": -123}"#).unwrap();
    let val = json_val.f64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": "123"}"#).unwrap();
    let val = json_val.f64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": {}}"#).unwrap();
    let val = json_val.f64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": []}"#).unwrap();
    let val = json_val.f64("number");
    assert!(val.is_none());
}

#[cfg(feature = "integer")]
#[test]
fn test_i64() {
    let json_val = parse_json(r#"{"number": 123}"#).unwrap();
    let val = json_val.i64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": -123}"#).unwrap();
    let val = json_val.i64("number").unwrap();
    assert_eq!(val, -123);

    let json_val = parse_json(r#"{"number": 1.23}"#).unwrap();
    let val = json_val.i64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": -1.23}"#).unwrap();
    let val = json_val.i64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": "123"}"#).unwrap();
    let val = json_val.i64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": {}}"#).unwrap();
    let val = json_val.i64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": []}"#).unwrap();
    let val = json_val.i64("number");
    assert!(val.is_none());
}

#[cfg(feature = "integer")]
#[test]
fn test_u64() {
    let json_val = parse_json(r#"{"number": 123}"#).unwrap();
    let val = json_val.u64("number").unwrap();
    assert_eq!(val, 123);

    let json_val = parse_json(r#"{"number": -123}"#).unwrap();
    let val = json_val.u64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": 1.23}"#).unwrap();
    let val = json_val.u64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": -1.23}"#).unwrap();
    let val = json_val.u64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": "123"}"#).unwrap();
    let val = json_val.u64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": {}}"#).unwrap();
    let val = json_val.u64("number");
    assert!(val.is_none());

    let json_val = parse_json(r#"{"number": []}"#).unwrap();
    let val = json_val.u64("number");
    assert!(val.is_none());
}
