#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::api::notification::Notification;

#[tauri::command]
fn notify() {
  Notification::new("studio.tauri.example")
  .title("New message")
  .body("You've got a new message.")
  .show();
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![notify])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
