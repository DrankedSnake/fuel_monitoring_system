use serde_json::Value;

pub trait AbstractModel {
    fn parse_f64(value: Option<&Value>) -> f64 {
        let value = match value.unwrap().to_string().replace("\"", "").parse::<f64>(){
            Ok(number) => number,
            Err(e) => panic!("Error during parsing f64 value{:?}", e),
        };
        value
    }

    fn parse_string(value: Option<&Value>) -> String{
        let value = match value.unwrap().to_string().replace("\"", "").parse(){
            Ok(text) => text,
            Err(e) => panic!("Error during parsing string value: {:?}", e),
        };
        value
    }
}