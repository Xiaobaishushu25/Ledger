use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub sort_order: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    pub date: String,
    pub amount: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
    pub image_count: i64,
    pub balance: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub id: i64,
    pub date: String,
    pub amount: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
    pub images: Vec<ImageRow>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageRow {
    pub id: i64,
    pub file_path: String,
    pub file_name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub total_income: i64,
    pub total_expense: i64,
    pub balance: i64,
    pub by_category: Vec<CategoryStat>,
    pub by_month: Vec<MonthStat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryStat {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthStat {
    pub month: String,
    pub income: i64,
    pub expense: i64,
}
