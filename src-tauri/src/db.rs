use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

pub struct DbState(pub Mutex<Connection>);

/// 便携版：数据跟随 exe 同级 data 目录（exe所在目录/data）
pub fn data_dir() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return parent.join("data");
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).join("data")
}

// 兼容旧接口，保留签名但忽略 AppHandle，统一走 data_dir
pub fn app_data_dir(_app: &tauri::AppHandle) -> PathBuf {
    data_dir()
}

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &dst_path)?;
        } else {
            let _ = std::fs::copy(entry.path(), dst_path);
        }
    }
    Ok(())
}

pub fn init_db(app: &tauri::AppHandle) -> Connection {
    let dir = data_dir();
    std::fs::create_dir_all(&dir).unwrap();
    let db_path = dir.join("lab-ledger.db");

    // 一次性迁移：若新位置没有数据但旧 appData 有，则自动拷贝
    if !db_path.exists() {
        if let Ok(old_dir) = app.path().app_data_dir() {
            let old_db = old_dir.join("lab-ledger.db");
            if old_db.exists() {
                let _ = std::fs::copy(&old_db, &db_path);
                let _ = std::fs::copy(old_dir.join("lab-ledger.db-wal"), dir.join("lab-ledger.db-wal"));
                let _ = std::fs::copy(old_dir.join("lab-ledger.db-shm"), dir.join("lab-ledger.db-shm"));
                let old_img = old_dir.join("images");
                let new_img = dir.join("images");
                if old_img.exists() && !new_img.exists() {
                    let _ = copy_dir_recursive(&old_img, &new_img);
                }
            }
        }
    }

    let try_init = |conn: &Connection| -> rusqlite::Result<()> {
        conn.execute_batch(
            r#"
            PRAGMA journal_mode=WAL;
            PRAGMA foreign_keys=ON;
            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                "type" TEXT CHECK("type" IN ('income','expense')) NOT NULL,
                sort_order INTEGER NOT NULL DEFAULT 99,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                amount INTEGER NOT NULL,
                "type" TEXT CHECK("type" IN ('income','expense')) NOT NULL,
                category_id INTEGER REFERENCES categories(id),
                note TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS transaction_images (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
                file_path TEXT NOT NULL,
                file_name TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT);
            "#,
        )
    };

    let conn = Connection::open(&db_path).expect("open db");
    if let Err(e) = try_init(&conn) {
        eprintln!("DB init failed (likely old schema), resetting DB: {}", e);
        drop(conn);
        let _ = std::fs::remove_file(&db_path);
        let conn2 = Connection::open(&db_path).expect("reopen db");
        try_init(&conn2).expect("init db after reset");
        let count: i64 = conn2.query_row("SELECT COUNT(*) FROM categories", [], |r| r.get(0)).unwrap_or(0);
        if count == 0 {
            let now = chrono::Utc::now().to_rfc3339();
            let seeds = vec![
                ("耗材", "expense", 1),
                ("设备", "expense", 2),
                ("差旅", "expense", 3),
                ("劳务", "expense", 4),
                ("测试费", "expense", 5),
                ("其他", "expense", 6),
                ("经费拨款", "income", 7),
                ("其他收入", "income", 8),
            ];
            for (name, typ, order) in &seeds {
                let _ = conn2.execute(
                    r#"INSERT OR IGNORE INTO categories (name, "type", sort_order, created_at) VALUES (?1, ?2, ?3, ?4)"#,
                    rusqlite::params![name, typ, order, now],
                );
            }
        }
        return conn2;
    }

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM categories", [], |r| r.get(0))
        .unwrap_or(0);
    if count == 0 {
        let now = chrono::Utc::now().to_rfc3339();
        let seeds = vec![
            ("耗材", "expense", 1),
            ("设备", "expense", 2),
            ("差旅", "expense", 3),
            ("劳务", "expense", 4),
            ("测试费", "expense", 5),
            ("其他", "expense", 6),
            ("经费拨款", "income", 7),
            ("其他收入", "income", 8),
        ];
        for (name, typ, order) in &seeds {
            let _ = conn.execute(
                r#"INSERT OR IGNORE INTO categories (name, "type", sort_order, created_at) VALUES (?1, ?2, ?3, ?4)"#,
                rusqlite::params![name, typ, order, now],
            );
        }
    }
    conn
}
