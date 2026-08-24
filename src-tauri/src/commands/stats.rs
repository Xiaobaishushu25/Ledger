use crate::db::DbState;
use crate::models::{Stats, CategoryStat, MonthStat};
use tauri::State;

#[tauri::command]
pub fn get_stats(db: State<DbState>) -> Result<Stats, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let total_income: i64 = conn.query_row(r#"SELECT COALESCE(SUM(amount),0) FROM transactions WHERE "type"='income'"#, [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let total_expense: i64 = conn.query_row(r#"SELECT COALESCE(SUM(amount),0) FROM transactions WHERE "type"='expense'"#, [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let balance = total_income - total_expense;

    let mut stmt = conn.prepare(r#"SELECT COALESCE(c.name,'未分类'), t."type", COALESCE(SUM(t.amount),0) FROM transactions t LEFT JOIN categories c ON c.id=t.category_id GROUP BY c.name, t."type" ORDER BY SUM(t.amount) DESC"#).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(CategoryStat{ name: r.get(0)?, type_: r.get(1)?, total: r.get(2)? })).map_err(|e| e.to_string())?;
    let mut by_category = Vec::new();
    for r in rows { by_category.push(r.map_err(|e| e.to_string())?); }

    let mut stmt2 = conn.prepare(r#"SELECT substr(date,1,7) as mon, COALESCE(SUM(CASE WHEN "type"='income' THEN amount ELSE 0 END),0), COALESCE(SUM(CASE WHEN "type"='expense' THEN amount ELSE 0 END),0) FROM transactions GROUP BY mon ORDER BY mon DESC LIMIT 12"#).map_err(|e| e.to_string())?;
    let rows2 = stmt2.query_map([], |r| Ok(MonthStat{ month: r.get(0)?, income: r.get(1)?, expense: r.get(2)? })).map_err(|e| e.to_string())?;
    let mut by_month = Vec::new();
    for r in rows2 { by_month.push(r.map_err(|e| e.to_string())?); }

    Ok(Stats{ total_income, total_expense, balance, by_category, by_month })
}

#[derive(serde::Serialize)]
pub struct DashboardSummary { pub balance: i64, pub month_income: i64, pub month_expense: i64 }

#[tauri::command]
pub fn get_dashboard_summary(db: State<DbState>) -> Result<DashboardSummary, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let balance: i64 = conn.query_row(r#"SELECT COALESCE(SUM(CASE WHEN "type"='income' THEN amount ELSE -amount END),0) FROM transactions"#, [], |r| r.get(0)).map_err(|e| e.to_string())?;
    let month = chrono::Utc::now().format("%Y-%m").to_string();
    let month_income: i64 = conn.query_row(r#"SELECT COALESCE(SUM(amount),0) FROM transactions WHERE "type"='income' AND substr(date,1,7)=?1"#, rusqlite::params![month], |r| r.get(0)).map_err(|e| e.to_string())?;
    let month_expense: i64 = conn.query_row(r#"SELECT COALESCE(SUM(amount),0) FROM transactions WHERE "type"='expense' AND substr(date,1,7)=?1"#, rusqlite::params![month], |r| r.get(0)).map_err(|e| e.to_string())?;
    Ok(DashboardSummary{ balance, month_income, month_expense })
}
