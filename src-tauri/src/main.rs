// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod db;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::tests::tests::get_tests,
            commands::clients::clients::get_clients,
            commands::clients::clients::post_clients,
            commands::clients::clients::update_client,
            commands::clients::clients::delete_client,
            commands::receipts::receipts::get_receipts,
            commands::receipts::receipts::post_receipts,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
