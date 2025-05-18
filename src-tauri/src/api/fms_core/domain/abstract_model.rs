use std::collections::HashMap;

use serde_json::{Value, Map};

pub trait AbstractModel {
    fn escape_double_quotes(value: Option<&Value>) -> String{
        if value.is_some(){
            value.unwrap().to_string().replace("\"", "")
        } else {
            String::from("")
        }
    }
    
    fn parse_f64(value: Option<&Value>) -> f64 {
        let value = match Self::escape_double_quotes(value).parse::<f64>(){
            Ok(number) => number,
            Err(e) => panic!("Error during parsing f64 value{:?}", e),
        };
        value
    }

    fn parse_f64_from_dict(map: &HashMap<String, Value>, key: &str) -> f64 {
        println!("Key: {key}");
        let value = match Self::escape_double_quotes(map.get(key)).parse::<f64>(){
            Ok(number) => number,
            Err(e) => panic!("Error during parsing f64 value{:?}", e),
        };
        println!("Succesfully parsed");
        value
    }

    fn parse_string(value: Option<&Value>) -> String{
        let value = match Self::escape_double_quotes(value).parse(){
            Ok(text) => text,
            Err(e) => panic!("Error during parsing string value: {:?}", e),
        };
        value
    }

    fn parse_i32(value: Option<&Value>) -> i32 {
        let value = match Self::escape_double_quotes(value).parse::<i32>(){
            Ok(number) => number,
            Err(e) => panic!("Error during parsing i32 value{:?}", e),
        };
        value
    }

    fn round_f64(value: f64) -> f64{
        format!("{:.3}", value).parse::<f64>().unwrap()
    }

    fn from_json_to_object(value: Option<&Value>) ->&Map<String, Value>{
        value.unwrap().as_object().unwrap()
    }

    fn from_number_to_i64(value: Option<&Value>) -> i64{
        value.unwrap().as_i64().unwrap()
    }
}

