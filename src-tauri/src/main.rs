#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use db::{init_db, DbState};
use std::sync::Mutex;
use tauri::Manager;

mod db;
mod models;
mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let conn = init_db(app.handle());
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::category::list_categories,
            commands::category::create_category,
            commands::category::update_category,
            commands::category::delete_category,
            commands::transaction::list_transactions,
            commands::transaction::create_transaction,
            commands::transaction::update_transaction,
            commands::transaction::delete_transaction,
            commands::transaction::get_transaction,
            commands::stats::get_stats,
            commands::stats::get_dashboard_summary,
            commands::image::save_image,
            commands::image::save_image_bytes,
            commands::image::save_image_by_path,
            commands::image::delete_image,
            commands::export::export_excel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



