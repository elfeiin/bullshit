#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bullshit::Action;

#[tauri::command]
fn issue(action: Action) {

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![issue])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
