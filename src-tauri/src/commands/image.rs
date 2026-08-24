use crate::db::{DbState, data_dir};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn save_image(_app: AppHandle, db: State<DbState>, transaction_id: Option<i64>, transactionId: Option<i64>, file_name: Option<String>, fileName: Option<String>, data_base64: String) -> Result<String, String> {
    let tid = transaction_id.or(transactionId).ok_or("missing transaction_id")?;
    let fname = file_name.or(fileName).unwrap_or_else(|| "image.png".to_string());
    let bytes = BASE64.decode(data_base64.trim()).map_err(|e| e.to_string())?;
    {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        let exists: i64 = conn.query_row("SELECT COUNT(*) FROM transactions WHERE id=?1", rusqlite::params![tid], |r| r.get(0)).map_err(|e| e.to_string())?;
        if exists==0 { return Err("流水不存在".into()); }
    }
    let dir = data_dir().join("images").join(Utc::now().format("%Y-%m").to_string());
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let ext = std::path::Path::new(&fname).extension().and_then(|s| s.to_str()).unwrap_or("png");
    let stored = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let path = dir.join(&stored);
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    let path_str = path.to_string_lossy().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO transaction_images (transaction_id, file_path, file_name, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![tid, path_str, fname, now]).map_err(|e| e.to_string())?;
    Ok(path_str)
}

#[tauri::command]
pub fn delete_image(db: State<DbState>, image_id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let path: Option<String> = conn.query_row("SELECT file_path FROM transaction_images WHERE id=?1", rusqlite::params![image_id], |r| r.get(0)).ok();
    conn.execute("DELETE FROM transaction_images WHERE id=?1", rusqlite::params![image_id]).map_err(|e| e.to_string())?;
    if let Some(p) = path { let _ = std::fs::remove_file(p); }
    Ok(())
}
#[tauri::command]
pub fn save_image_bytes(_app: AppHandle, db: State<DbState>, transaction_id: Option<i64>, transactionId: Option<i64>, file_name: Option<String>, fileName: Option<String>, data: Vec<u8>) -> Result<String, String> {
    let tid = transaction_id.or(transactionId).ok_or("missing transaction_id")?;
    let fname = file_name.or(fileName).unwrap_or_else(|| "image.png".to_string());
    {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        let exists: i64 = conn.query_row("SELECT COUNT(*) FROM transactions WHERE id=?1", rusqlite::params![tid], |r| r.get(0)).map_err(|e| e.to_string())?;
        if exists==0 { return Err("流水不存在".into()); }
    }
    let dir = data_dir().join("images").join(Utc::now().format("%Y-%m").to_string());
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let ext = std::path::Path::new(&fname).extension().and_then(|s| s.to_str()).unwrap_or("png");
    let stored = format!("{}.{}", uuid::Uuid::new_v4(), ext);
    let path = dir.join(&stored);
    std::fs::write(&path, &data).map_err(|e| e.to_string())?;
    let path_str = path.to_string_lossy().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO transaction_images (transaction_id, file_path, file_name, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![tid, path_str, fname, now]).map_err(|e| e.to_string())?;
    Ok(path_str)
}

#[tauri::command]
pub fn save_image_by_path(_app: AppHandle, db: State<DbState>, transaction_id: Option<i64>, transactionId: Option<i64>, source_path: String) -> Result<String, String> {
    let tid = transaction_id.or(transactionId).ok_or("missing transaction_id")?;
    let bytes = std::fs::read(&source_path).map_err(|e| e.to_string())?;
    let file_name = std::path::Path::new(&source_path).file_name().and_then(|s| s.to_str()).unwrap_or("image.png").to_string();
    return save_image_bytes(_app, db, Some(tid), None, Some(file_name), None, bytes);
}
