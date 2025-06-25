pub mod project;
pub mod hex;
pub mod java;
pub mod android;

#[tauri::command]
fn create_project(path: &str) -> String {
    let project_id = project::Project::create_project_from_path(path);
    project_id
}

#[tauri::command]
fn project_get_type(project_id: &str) -> String {
    match project::Project::query_type(project_id) {
        Some(project_type) => format!("{:?}", project_type),
        None => "Unknown".to_string(),
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_project, 
            project_get_type,
            hex::hex_project_get_file_size,
            hex::hex_project_get_total_pages,
            hex::hex_project_read_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
