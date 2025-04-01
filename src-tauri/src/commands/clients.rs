pub mod clients {
    use app::Client;

    use crate::db::postgres::postgres::{get_data_from_db, insert_data_to_db, update_data_to_db, delete_data_to_db};

    #[tauri::command]
    pub async fn get_clients() -> Result<Vec<Client>, String> {
        let res = get_data_from_db::<Client>("clients").await;
        match res {
            Ok(response) => Ok(response),
            Err(error) => Err(error.to_string()),
        }
    }

    #[tauri::command]
    pub async fn post_clients(client: String) -> Result<Vec<Client>, String> {
        let client_result = insert_data_to_db::<Client>("clients", client).await;
        if let Err(err) = client_result {
            return Err(format!("Error inserting client data: {}", err.to_string()));
        };
        Ok(client_result.unwrap())
    }

    #[tauri::command]
    pub async fn update_client(
        id_value: String,
        updated_client: String,
    ) -> Result<Vec<Client>, String> {
        let client_result = update_data_to_db("clients", "id", &id_value, updated_client).await;
        match client_result {
            Ok(response) => Ok(response),
            Err(err) => Err(err.to_string()),
        }
    }

    #[tauri::command]
    pub async fn delete_client(
        id_value: String,
    ) -> Result<Vec<Client>, String> {
        let client_result = delete_data_to_db("clients", "id", &id_value).await;
        match client_result {
            Ok(response) => Ok(response),
            Err(err) => Err(err.to_string()),
        }
    }
}
