use std::{collections::HashMap, sync::Arc};

use rocket::tokio::sync::Mutex;

pub struct WeatherSingleFlight{
    locks: Mutex<HashMap<String, Arc<Mutex<()>>>>
}

impl WeatherSingleFlight{
    pub fn new() -> Self{
        Self { locks: Mutex::new(HashMap::new())}
    }


    pub async fn get_lock(&self, key: &str) -> Arc<Mutex<()>>{
        let mut locks = self.locks.lock().await;

        let result = locks.entry(key.to_string())
        .or_insert_with(|| Arc::new(Mutex::new(())));

        result.clone()
    }
}