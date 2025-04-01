pub mod postgres {
    use app::Receipt;
    use app::{SUPABASE_API_KEY, SUPABASE_DB_URL};
    use postgrest::Postgrest;
    use reqwest;
    use serde::de::DeserializeOwned;
    use serde::ser::Serialize;
    use supabase_storage::{config::SupabaseConfig, Storage};

    fn create_pg_client() -> Postgrest {
        let db_url = SUPABASE_DB_URL;
        let client = Postgrest::new(db_url).insert_header("apikey", SUPABASE_API_KEY);
        client
    }

    pub async fn connect_db_storage() {
        let config = SupabaseConfig::default();
        let storage = Storage::new_with_config(config);
        let bucket_name = "emerald_bucker";
        let response = storage
            .from()
            .get_bucket_details(bucket_name)
            .execute()
            .await
            .unwrap();
        println!("{:?}", response);
    }

    pub async fn get_data_from_db<T>(table_name: &str) -> reqwest::Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let client = create_pg_client();
        let resp = client.from(table_name).select("*").execute().await?;
        let body = resp.json().await;
        body
    }

    pub async fn get_receipts_by_field(
        field_name: &str,
        field_value: &str,
    ) -> reqwest::Result<Vec<Receipt>>
    where
        Receipt: DeserializeOwned,
    {
        let client = create_pg_client();
        let resp = client
            .from("receipts")
            .eq(field_name, field_value)
            .select("*")
            .execute()
            .await?;
        let body = resp.json().await?;
        Ok(body)
    }

    pub async fn insert_data_to_db<T>(
        table_name: &str,
        data: String,
    ) -> Result<Vec<T>, reqwest::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let client = create_pg_client();
        let resp = client
            .from(table_name)
            .insert(&data)
            .select("*")
            .execute()
            .await;
        match resp {
            Ok(response) => {
                let body = response.json().await;
                body
            }
            Err(err) => Err(err),
        }
    }

    pub async fn update_data_to_db<T>(
        table_name: &str,
        id_field_name: &str,
        id_value: &str,
        updated_data: String,
    ) -> Result<Vec<T>, reqwest::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let client = create_pg_client();
        let resp = client
            .from(table_name)
            .eq(id_field_name, id_value)
            .update(updated_data)
            .select("*")
            .execute()
            .await;
        match resp {
            Ok(response) => {
                let body = response.json().await;
                body
            }
            Err(err) => Err(err),
        }
    }

    pub async fn delete_data_to_db<T>(
        table_name: &str,
        id_field_name: &str,
        id_value: &str,
    ) -> Result<Vec<T>, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let client = create_pg_client();
        let resp = client
            .from(table_name)
            .eq(id_field_name, id_value)
            .delete()
            .execute()
            .await;
        match resp {
            Ok(response) => {
                let body = response.json().await;
                body
            }
            Err(err) => Err(err),
        }
    }

    #[tokio::test]
    async fn get_data_from_db_test() {
        let test_data = get_data_from_db::<app::Test>("test").await.unwrap();
        print!("{:?}", test_data);
        assert_ne!(test_data.len(), 0);
    }

    #[tokio::test]
    async fn update_data_to_db_test() {
        let test_data = update_data_to_db::<app::Test>(
            "test",
            "id",
            "2",
            "{\"name\": \"Changed2\"}".to_string(),
        )
        .await;
        match test_data {
            Ok(response) => {
                println!("{:?}", response);
                assert_ne!(response.len(), 0);
            }
            Err(err) => {
                println!("{err}");
                panic!("Llego al fallo")
            }
        }
    }

    #[tokio::test]
    async fn delete_data_to_db_test() {
        let test_data = delete_data_to_db::<app::Test>("test", "id", "3").await;
        match test_data {
            Ok(response) => {
                println!("{:?}", response);
                assert_ne!(response.len(), 0);
            }
            Err(err) => {
                println!("{err}");
                panic!("Llego al fallo")
            }
        }
    }
}
