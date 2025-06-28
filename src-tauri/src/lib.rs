pub mod project;
pub mod hex;
pub mod java;
pub mod android;
mod java_analyzer;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            project::create_project,
            project::project_get_type,
            hex::hex_project_get_file_size,
            hex::hex_project_get_total_pages,
            hex::hex_project_read_page,
            java::java_project_list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
