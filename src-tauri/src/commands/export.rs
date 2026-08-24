use crate::db::DbState;
use rust_xlsxwriter::{Workbook, Format, Color};
use tauri::State;

#[tauri::command]
pub fn export_excel(db: State<DbState>, keyword: Option<String>, type_: Option<String>, category_id: Option<i64>, date_from: Option<String>, date_to: Option<String>) -> Result<String, String> {
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

    let sql = format!(r#"SELECT t.date, t."type", COALESCE(c.name,''), t.amount, t.note FROM transactions t LEFT JOIN categories c ON c.id=t.category_id {} ORDER BY t.date ASC, t.id ASC"#, where_sql);
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let p_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref() as &dyn rusqlite::ToSql).collect();
    let rows = stmt.query_map(p_refs.as_slice(), |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, i64>(3)?, r.get::<_, String>(4)?))).map_err(|e| e.to_string())?;

    let mut data: Vec<(String,String,String,i64,String)> = Vec::new();
    for r in rows { data.push(r.map_err(|e| e.to_string())?); }

    let download = dirs::download_dir().or_else(|| dirs::document_dir()).unwrap_or_else(|| std::env::temp_dir());
    let filename = format!("实验室流水_{}.xlsx", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    let path = download.join(&filename);
    let path_str = path.to_string_lossy().to_string();

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.set_name("流水").map_err(|e| e.to_string())?;

    let header_fmt = Format::new().set_bold().set_background_color(Color::RGB(0xEEF0FF));
    let headers = ["日期","收支","类别","金额(元)","说明"];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_with_format(0, col as u16, *h, &header_fmt).map_err(|e| e.to_string())?;
    }
    for (idx, (date, type_, cat, amount, note)) in data.iter().enumerate() {
        let row = (idx+1) as u32;
        sheet.write(row, 0, date.as_str()).map_err(|e| e.to_string())?;
        sheet.write(row, 1, if type_=="income" {"收入"} else {"支出"}).map_err(|e| e.to_string())?;
        sheet.write(row, 2, cat.as_str()).map_err(|e| e.to_string())?;
        sheet.write_number(row, 3, *amount as f64 / 100.0).map_err(|e| e.to_string())?;
        sheet.write(row, 4, note.as_str()).map_err(|e| e.to_string())?;
    }
    let total_row = (data.len()+1) as u32;
    sheet.write_with_format(total_row, 0, "合计", &header_fmt).map_err(|e| e.to_string())?;
    let income: i64 = data.iter().filter(|(_,t,_,_,_)| t=="income").map(|(_,_,_,a,_)| *a).sum();
    let expense: i64 = data.iter().filter(|(_,t,_,_,_)| t=="expense").map(|(_,_,_,a,_)| *a).sum();
    sheet.write(total_row, 1, format!("收入{}笔 支出{}笔", data.iter().filter(|(_,t,_,_,_)| t=="income").count(), data.iter().filter(|(_,t,_,_,_)| t=="expense").count())).map_err(|e| e.to_string())?;
    sheet.write_number(total_row, 3, (income - expense) as f64 / 100.0).map_err(|e| e.to_string())?;

    sheet.set_column_width(0, 12).map_err(|e| e.to_string())?;
    sheet.set_column_width(1, 10).map_err(|e| e.to_string())?;
    sheet.set_column_width(2, 14).map_err(|e| e.to_string())?;
    sheet.set_column_width(3, 14).map_err(|e| e.to_string())?;
    sheet.set_column_width(4, 40).map_err(|e| e.to_string())?;

    workbook.save(&path).map_err(|e| e.to_string())?;
    Ok(path_str)
}
