#[tauri::command]
pub fn get_tankers() -> String {
    "HELLO FROM TANKS".to_string()
}