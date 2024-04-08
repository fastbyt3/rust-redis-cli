use redis::{Client, Commands, Connection};

use crate::config::Config;

pub struct Redis {
    client: Client,
}

impl Redis {
    pub fn new(cfg: Config) -> Result<Self, String> {
        let client = redis::Client::open(cfg.generate_url())
            .expect("Failed to make connection with redis server");
        Ok(Self { client })
    }

    pub fn get_connection(&self) -> Result<Connection, String> {
        self.client
            .get_connection()
            .map_err(|e| format!("Failed to get redis connection: {}", e))
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        let mut connection = self.get_connection().unwrap();
        connection
            .get(key)
            .map_err(|e| format!("Failed to get {}. Error => {}", key, e))
    }
}
