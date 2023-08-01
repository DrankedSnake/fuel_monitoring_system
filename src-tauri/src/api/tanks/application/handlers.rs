use std::collections::HashMap;
use serde::{Serialize, Serializer};


pub enum Value<'a>{
    Str(&'a str),
    Int(i32),
    Float(f32),
}

impl <'a>Serialize for Value<'a>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where 
        S: Serializer,
    {
        match *self{
            Value::Str(s) => serializer.serialize_str(s),
            Value::Int(i) => serializer.serialize_i32(i),
            Value::Float(f) => serializer.serialize_f32(f),
        }
    }
}


#[tauri::command]
pub fn get_tanks<'a>() -> Vec<HashMap<&'a str, Value<'a>>>{
    let mut tanks: Vec<HashMap<&str, Value>> = Vec::new();
    tanks.push(HashMap::from([("name", Value::Str("A001")), ("available_volume", Value::Float(100.0)), ("current_volume", Value::Float(50.0))]));
    tanks.push(HashMap::from([("name", Value::Str("A002")), ("available_volume", Value::Float(120.0)), ("current_volume", Value::Float(40.0))]));
    tanks.push(HashMap::from([("name", Value::Str("A003")), ("available_volume", Value::Float(200.0)), ("current_volume", Value::Float(30.0))]));
    tanks.push(HashMap::from([("name", Value::Str("B001")), ("available_volume", Value::Float(300.0)), ("current_volume", Value::Float(20.0))]));
    tanks.push(HashMap::from([("name", Value::Str("B002")), ("available_volume", Value::Float(300.0)), ("current_volume", Value::Float(20.0))]));
    tanks.push(HashMap::from([("name", Value::Str("B004")), ("available_volume", Value::Float(100.0)), ("current_volume", Value::Float(60.0))]));
    tanks.push(HashMap::from([("name", Value::Str("B003")), ("available_volume", Value::Float(100.0)), ("current_volume", Value::Float(10.0))]));
    tanks.push(HashMap::from([("name", Value::Str("B005")), ("available_volume", Value::Float(200.0)), ("current_volume", Value::Float(45.0))]));
    return tanks;
}