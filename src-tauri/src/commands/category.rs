use crate::db::DbState;
use crate::models::Category;
use chrono::Utc;
use tauri::State;

#[tauri::command]
pub fn list_categories(db: State<DbState>) -> Result<Vec<Category>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(r#"SELECT id, name, "type", sort_order, created_at FROM categories ORDER BY sort_order, id"#).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(Category { id: row.get(0)?, name: row.get(1)?, type_: row.get(2)?, sort_order: row.get(3)?, created_at: row.get(4)? })
    }).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows { out.push(r.map_err(|e| e.to_string())?); }
    Ok(out)
}

#[tauri::command]
pub fn create_category(db: State<DbState>, name: String, type_: String) -> Result<i64, String> {
    if name.trim().is_empty() { return Err("类别名不能为空".into()); }
    if type_ != "income" && type_ != "expense" { return Err("类型错误".into()); }
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(r#"INSERT INTO categories (name, "type", sort_order, created_at) VALUES (?1, ?2, 99, ?3)"#, rusqlite::params![name.trim(), type_, now]).map_err(|e| {
        if e.to_string().contains("UNIQUE") { "类别名已存在".to_string() } else { e.to_string() }
    })?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn update_category(db: State<DbState>, id: i64, name: String) -> Result<(), String> {
    if name.trim().is_empty() { return Err("类别名不能为空".into()); }
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let n = conn.execute("UPDATE categories SET name=?1 WHERE id=?2", rusqlite::params![name.trim(), id]).map_err(|e| e.to_string())?;
    if n==0 { return Err("类别不存在".into()); }
    Ok(())
}

#[tauri::command]
pub fn delete_category(db: State<DbState>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let cnt: i64 = conn.query_row("SELECT COUNT(*) FROM transactions WHERE category_id=?1", rusqlite::params![id], |r| r.get(0)).map_err(|e| e.to_string())?;
    if cnt>0 { return Err("该类别已被流水使用，无法删除".into()); }
    let n = conn.execute("DELETE FROM categories WHERE id=?1", rusqlite::params![id]).map_err(|e| e.to_string())?;
    if n==0 { return Err("类别不存在".into()); }
    Ok(())
}
