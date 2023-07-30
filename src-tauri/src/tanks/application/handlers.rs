use std::collections::HashMap;


#[tauri::command]
pub fn get_tanks() -> Vec<HashMap<String, String>>{
    let mut tanks: Vec<HashMap<String, String>> = Vec::new();
    tanks.push(HashMap::from([("name".to_string(), "A001".to_string()), ("available_volume".to_string(), "100".to_string()), ("current_volume".to_string(), "50".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "A002".to_string()), ("available_volume".to_string(), "120".to_string()), ("current_volume".to_string(), "40".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "A003".to_string()), ("available_volume".to_string(), "200".to_string()), ("current_volume".to_string(), "30".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "B001".to_string()), ("available_volume".to_string(), "300".to_string()), ("current_volume".to_string(), "20".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "B002".to_string()), ("available_volume".to_string(), "300".to_string()), ("current_volume".to_string(), "20".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "B004".to_string()), ("available_volume".to_string(), "100".to_string()), ("current_volume".to_string(), "60".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "B003".to_string()), ("available_volume".to_string(), "100".to_string()), ("current_volume".to_string(), "10".to_string())]));
    tanks.push(HashMap::from([("name".to_string(), "B005".to_string()), ("available_volume".to_string(), "200".to_string()), ("current_volume".to_string(), "45".to_string())]));
    return tanks;
}