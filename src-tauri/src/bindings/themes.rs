#[tauri::command]
pub async fn query_settings_theme() -> String {
    return "light".to_string();
}
