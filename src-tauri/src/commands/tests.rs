pub mod tests {
    use app::Test;

    use crate::db::postgres::postgres::get_data_from_db;

    pub fn set_result<T>(result: reqwest::Result<T>) -> Result<T, ()> {
        result.map_err(|_error| ())
    }

    #[tauri::command]
    pub async fn get_tests() -> Result<Vec<Test>, String> {
        let res = get_data_from_db::<Test>("test").await;
        match res {
            Ok(response) => Ok(response),
            Err(error) => Err(error.to_string()),
        }
    }
}
