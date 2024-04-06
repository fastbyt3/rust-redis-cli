#[derive(Debug)]
pub struct Config {
    pub host_name: String,
    pub port: u16,
    pub auth: Option<String>,
}

impl Config {
    pub fn new(host_name: &str, port: &str, auth: Option<String>) -> Self {
        Self {
            host_name: host_name.to_owned(),
            port: port.parse::<u16>().expect("Failed to parse port number as an integer"),
            auth,
        }
    }

    pub fn generate_url(&self) -> String {
        match &self.auth {
            Some(passwd) => format!("redis://:{}@{}:{}/0", passwd, self.host_name, self.port),
            None => format!("redis://{}:{}/0", self.host_name, self.port),
        }
    }
}
