pub mod receipts {
    use app::{Item, ItemForm};
    use app::Receipt;
    use serde_json::to_string;

    use crate::db::postgres::postgres::{get_data_from_db, insert_data_to_db};

    #[tauri::command]
    pub async fn get_receipts() -> Result<Vec<Receipt>, String> {
        let res = get_data_from_db::<Receipt>("receipts").await;
        match res {
            Ok(response) => Ok(response),
            Err(error) => Err(error.to_string()),
        }
    }

    #[tauri::command]
    pub async fn post_receipts(receipt: String, items: Vec<Item>) -> Result<(Vec<Receipt>, Vec<Item>), String> {
        let receipt_result = insert_data_to_db::<Receipt>("receipts", receipt).await;
        if let Err(err) = &receipt_result {
            return Err(format!("Error inserting receipt data: {}", err));
        };
        let receipts = receipt_result.unwrap();
        let id = receipts[0].id;
        let new_items: Vec<ItemForm> = items.into_iter().map(|x| ItemForm {
            cut: x.cut,
            receipt_id: Some(id),
            color: x.color,
            height: x.height,
            dim_unit: x.dim_unit,
            jewel: x.jewel ,
            language: x.language,
            length: x.length,
            service: x.service,
            value: x.value,
            weight_unit: x.weight_unit,
            weight: x.weight,
            width: x.width,
        }).rev().collect();
        let items_json =
            to_string(&new_items).map_err(|e| format!("Serialization items error: {}", e))?;
        let items_result = insert_data_to_db::<Item>("items", items_json).await;
        match items_result {
            Err(err) => Err(format!("Error inserting items data: {}", err)),
            Ok(items) => Ok((receipts, items)),
        }
    }
}
