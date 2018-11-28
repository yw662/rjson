use std::vec::Vec;
use std::collections::BTreeMap;
use std::convert::From;
use std::any::Any;
use std::boxed::Box;
use crate::Value;
use crate::Array;
use crate::Object;
use crate::Null;
use crate::parse;

#[derive(Clone)]
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

impl<'a> Array<'a, JsonValue> for JsonArray {
    fn new<'b>() -> &'b mut Self {
        // FIXME: seems that `new` should not return a ref.
        Box::leak::<'b>(Box::new(JsonArray(Vec::new())))
    }
    fn push(&mut self, v: JsonValue) {
        self.0.push(v)
    }
    fn as_any(&self) -> &dyn Any {
        &self.0
    }
}

impl<'a> Object<'a, JsonValue> for JsonObject {
    fn new<'b>() -> &'b mut Self {
        // FIXME: again
        Box::leak::<'b>(Box::new(JsonObject(BTreeMap::new())))
    }
    fn insert(&mut self, k: String, v: JsonValue) {
        self.0.insert(k, v);
    }
    fn as_any(&self) -> &dyn Any {
        &self.0
    }
}

impl<'a> Null<'a, JsonValue> for JsonValue {
    fn new<'b>() -> &'b Self {
        &JsonValue::Null
    }
}
impl<'a> Value<'a> for JsonValue {}

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
impl<'a> From<&'a Array<'a, JsonValue>> for JsonValue {
    fn from(v: & Array<JsonValue>) -> Self {
        let r = v.as_any().downcast_ref::<Vec<JsonValue>>().unwrap();
        JsonValue::Array(r.clone())
    }
}
impl<'a> From<&'a Object<'a, JsonValue>> for JsonValue {
    fn from(v: & Object<JsonValue>) -> Self {
        let r = v.as_any().downcast_ref::<BTreeMap<String, JsonValue>>().unwrap();
        JsonValue::Object(r.clone())
    }
}
impl<'a> From<&'a Null<'a, JsonValue>> for JsonValue {
    fn from(_v: & Null<JsonValue>) -> Self {
        JsonValue::Null
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

