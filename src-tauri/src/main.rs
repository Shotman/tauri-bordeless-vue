#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
  other_val: usize,
}

#[tauri::command(with_window)]
async fn my_custom_command<M: tauri::Params>(
  window: tauri::Window<M>,
  number: usize,
) -> Result<CustomResponse, String> {
  println!("Called from {}", window.label());
  let result: Option<String> = Some("Test".to_owned());
  if let Some(message) = result {
    Ok(CustomResponse {
      message,
      other_val: 42 + number,
    })
  } else {
    Err("No result".into())
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
