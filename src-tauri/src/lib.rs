use serde::{Deserialize, Serialize};

pub const SUPABASE_PASSWORD: &str = "your supabase password";
pub const SUPABASE_API_KEY: &str = "Your Supabase API Key";
pub const SUPABASE_URL_STORAGE: &str = "Your supabase storage url";
pub const SUPABASE_DB_URL: &str = "Your supabase db url";

#[derive(Deserialize, Debug, Serialize)]
pub struct Test {
    id: u32,
    created_at: String,
    name: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct Receipt {
    pub id: u32,
    client_doc: String,
    entry_date: String,
    delivery_date: String,
    observations: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Client {
    doc: String,
    name: String,
    mobile: String,
    landline: String,
    address: String,
    email: String,
    discount: u8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Item {
    id: Option<u32>,
    pub jewel: String,
    pub receipt_id: Option<u32>,
    pub weight: f32,
    pub weight_unit: String,
    pub length: f32,
    pub width: f32,
    pub height: f32,
    pub dim_unit: String,
    pub color: String,
    pub cut: String,
    pub language: String,
    pub service: String,
    pub value: f32,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ItemForm {
    pub jewel: String,
    pub receipt_id: Option<u32>,
    pub weight: f32,
    pub weight_unit: String,
    pub length: f32,
    pub width: f32,
    pub height: f32,
    pub dim_unit: String,
    pub color: String,
    pub cut: String,
    pub language: String,
    pub service: String,
    pub value: f32,
}
