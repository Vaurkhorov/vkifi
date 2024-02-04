// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod wrappers;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      wrappers::initialise,
      wrappers::meta,
      wrappers::track,
      wrappers::preview,
      wrappers::snapshot,
      wrappers::log,
      wrappers::revert,
      wrappers::register,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
