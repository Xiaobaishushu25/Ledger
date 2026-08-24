use crate::db::DbState;
use crate::models::{Transaction, TransactionDetail, ImageRow};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub keyword: Option<String>,
    pub type_: Option<String>,
    pub category_id: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TxPayload {
    pub date: String,
    pub amount: f64,
    #[serde(rename = "type_")]
    pub type_: String,
    pub category_id: Option<i64>,
    pub note: String,
}

#[derive(Debug, Serialize)]
pub struct ListResult {
    pub items: Vec<Transaction>,
    pub total: i64,
    pub balance: i64,
}

#[tauri::command]
pub fn list_transactions(db: State<DbState>, keyword: Option<String>, type_: Option<String>, category_id: Option<i64>, date_from: Option<String>, date_to: Option<String>, page: Option<i64>, page_size: Option<i64>) -> Result<ListResult, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut where_clauses: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(kw) = keyword.as_ref().filter(|s| !s.trim().is_empty()) {
        where_clauses.push("t.note LIKE ?".to_string());
        params.push(Box::new(format!("%{}%", kw.trim())));
    }
    if let Some(tp) = type_.as_ref().filter(|s| !s.is_empty()) {
        where_clauses.push(r#"t."type" = ?"#.to_string());
        params.push(Box::new(tp.clone()));
    }
    if let Some(cid) = category_id {
        where_clauses.push("t.category_id = ?".to_string());
        params.push(Box::new(cid));
    }
    if let Some(df) = date_from.as_ref().filter(|s| !s.is_empty()) {
        where_clauses.push("t.date >= ?".to_string());
        params.push(Box::new(df.clone()));
    }
    if let Some(dt) = date_to.as_ref().filter(|s| !s.is_empty()) {
        where_clauses.push("t.date <= ?".to_string());
        params.push(Box::new(dt.clone()));
    }

    let where_sql = if where_clauses.is_empty() { "".to_string() } else { format!("WHERE {}", where_clauses.join(" AND ")) };

    let count_sql = format!(r#"SELECT COUNT(*) FROM transactions t {}"#, where_sql);
    let total: i64 = {
        let mut stmt = conn.prepare(&count_sql).map_err(|e| e.to_string())?;
        let p_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref() as &dyn rusqlite::ToSql).collect();
        stmt.query_row(p_refs.as_slice(), |r| r.get(0)).map_err(|e| e.to_string())?
    };

    let balance: i64 = conn.query_row(r#"SELECT COALESCE(SUM(CASE WHEN "type"='income' THEN amount ELSE -amount END),0) FROM transactions"#, [], |r| r.get(0)).map_err(|e| e.to_string())?;

    let fetch_sql = format!(r#"SELECT t.id, t.date, t.amount, t."type", t.category_id, c.name, t.note, t.created_at, t.updated_at, (SELECT COUNT(*) FROM transaction_images ti WHERE ti.transaction_id=t.id) as img_cnt FROM transactions t LEFT JOIN categories c ON c.id=t.category_id {} ORDER BY t.date ASC, t.id ASC"#, where_sql);
    let mut stmt = conn.prepare(&fetch_sql).map_err(|e| e.to_string())?;
    let p_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref() as &dyn rusqlite::ToSql).collect();
    let rows = stmt.query_map(p_refs.as_slice(), |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, i64>(2)?, row.get::<_, String>(3)?, row.get::<_, Option<i64>>(4)?, row.get::<_, Option<String>>(5)?, row.get::<_, String>(6)?, row.get::<_, String>(7)?, row.get::<_, String>(8)?, row.get::<_, i64>(9)?))
    }).map_err(|e| e.to_string())?;

    let mut all: Vec<Transaction> = Vec::new();
    let mut bal: i64 = 0;
    for r in rows {
        let (id, date, amount, type_, category_id, category_name, note, created_at, updated_at, img_cnt) = r.map_err(|e| e.to_string())?;
        if type_=="income" { bal += amount; } else { bal -= amount; }
        all.push(Transaction { id, date, amount, type_, category_id, category_name, note, created_at, updated_at, image_count: img_cnt, balance: Some(bal) });
    }

    all.reverse();
    let p = page.unwrap_or(1).max(1);
    let ps = page_size.unwrap_or(15).clamp(5, 100);
    let start = ((p-1)*ps) as usize;
    let items: Vec<Transaction> = all.into_iter().skip(start).take(ps as usize).collect();

    Ok(ListResult { items, total, balance })
}

#[tauri::command]
pub fn create_transaction(db: State<DbState>, payload: TxPayload) -> Result<i64, String> {
    if payload.amount <= 0.0 { return Err("金额必须大于0".into()); }
    if payload.type_ != "income" && payload.type_ != "expense" { return Err("类型错误".into()); }
    if payload.date.trim().is_empty() { return Err("日期不能为空".into()); }
    let cents = (payload.amount * 100.0).round() as i64;
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(r#"INSERT INTO transactions (date, amount, "type", category_id, note, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
        rusqlite::params![payload.date, cents, payload.type_, payload.category_id, payload.note.trim(), now, now]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn update_transaction(db: State<DbState>, id: i64, payload: TxPayload) -> Result<(), String> {
    if payload.amount <= 0.0 { return Err("金额必须大于0".into()); }
    let cents = (payload.amount * 100.0).round() as i64;
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    let n = conn.execute(r#"UPDATE transactions SET date=?1, amount=?2, "type"=?3, category_id=?4, note=?5, updated_at=?6 WHERE id=?7"#,
        rusqlite::params![payload.date, cents, payload.type_, payload.category_id, payload.note.trim(), now, id]).map_err(|e| e.to_string())?;
    if n==0 { return Err("记录不存在".into()); }
    Ok(())
}

#[tauri::command]
pub fn delete_transaction(db: State<DbState>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let paths: Vec<String> = {
        let mut stmt = conn.prepare("SELECT file_path FROM transaction_images WHERE transaction_id=?1").map_err(|e| e.to_string())?;
        let rows = stmt.query_map(rusqlite::params![id], |r| r.get(0)).map_err(|e| e.to_string())?;
        rows.filter_map(|r| r.ok()).collect()
    };
    conn.execute("DELETE FROM transactions WHERE id=?1", rusqlite::params![id]).map_err(|e| e.to_string())?;
    for p in paths { let _ = std::fs::remove_file(p); }
    Ok(())
}

#[tauri::command]
pub fn get_transaction(db: State<DbState>, id: i64) -> Result<TransactionDetail, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let (tid, date, amount, type_, category_id, category_name, note, created_at, updated_at) = conn.query_row(
        r#"SELECT t.id, t.date, t.amount, t."type", t.category_id, c.name, t.note, t.created_at, t.updated_at FROM transactions t LEFT JOIN categories c ON c.id=t.category_id WHERE t.id=?1"#,
        rusqlite::params![id], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(2)?, r.get::<_, String>(3)?, r.get::<_, Option<i64>>(4)?, r.get::<_, Option<String>>(5)?, r.get::<_, String>(6)?, r.get::<_, String>(7)?, r.get::<_, String>(8)?))
    ).map_err(|_| "记录不存在".to_string())?;

    let mut stmt = conn.prepare("SELECT id, file_path, file_name, created_at FROM transaction_images WHERE transaction_id=?1 ORDER BY id").map_err(|e| e.to_string())?;
    let rows = stmt.query_map(rusqlite::params![id], |r| Ok(ImageRow{ id: r.get(0)?, file_path: r.get(1)?, file_name: r.get(2)?, created_at: r.get(3)? })).map_err(|e| e.to_string())?;
    let mut images = Vec::new();
    for r in rows { images.push(r.map_err(|e| e.to_string())?); }

    Ok(TransactionDetail{ id: tid, date, amount, type_, category_id, category_name, note, created_at, updated_at, images })
}
