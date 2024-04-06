use std::u64;

use redis::{Commands, Connection};

#[derive(Debug)]
enum CliCommands {
    KEYS,
    GET,
    SET,
    SETEX,
    TTL,
    DEL,
    QUIT,
}

impl TryFrom<&str> for CliCommands {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "KEYS" | "keys" => Ok(Self::KEYS),
            "GET" | "get" => Ok(Self::GET),
            "SET" | "set" => Ok(Self::SET),
            "SETEX" | "setex" => Ok(Self::SETEX),
            "TTL" | "ttl" => Ok(Self::TTL),
            "DEL" | "del" => Ok(Self::DEL),
            "QUIT" | "quit" => Ok(Self::QUIT),
            _ => Err(format!("Failed to parse command: {}", value)),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    command: CliCommands,
    arguments: Vec<String>,
}

impl Command {
    pub fn new(input: String) -> Result<Self, String> {
        let mut split = input.split_whitespace();
        let cmd = CliCommands::try_from(split.next().unwrap())?;
        let rest = split.map(|s| s.to_owned()).collect::<Vec<String>>();

        Ok(Self {
            command: cmd,
            arguments: rest,
        })
    }

    pub fn execute(&self, mut connection: Connection) -> Result<String, String> {
        match self.command {
            CliCommands::KEYS => {
                let pattern = self.arguments.get(0).unwrap();
                let keys: Vec<String> = connection.keys(pattern).map_err(|e| { format!("Failed to find keys: {}, Error: {}", pattern, e) })?;
                return Ok(format!("Matching keys: {:?}", keys));
            },
            CliCommands::GET => {
                let key = self.arguments.get(0).unwrap();
                let value: String = connection.get(key).map_err(|e| {format!("Failed to get key: {}, Error: {}", key, e)})?;
                return Ok(format!("GOT :: {} => {}", key, value));
            },
            CliCommands::DEL => {
                let key = self.arguments.get(0).unwrap();
                let _ = connection.del(key).map_err(|e| { format!("Failed to delete key: {}. Error: {}", key, e) })?;
                return Ok(format!("Successfully deleted: {}", key));
            },
            CliCommands::SET => {
                let key = self.arguments.get(0).unwrap();
                let value = self.arguments.get(1).unwrap();
                let _ = connection.set(key, value).map_err(|e| { format!("Failed to set KV: {} -> {}. Error: {}", key, value, e) })?;
                return Ok(format!("Successfully set: {}:{}", key, value));
            },
            CliCommands::SETEX => {
                let key = self.arguments.get(0).unwrap();
                let value = self.arguments.get(1).unwrap();
                let seconds = self.arguments.get(2).unwrap().parse::<u64>().unwrap();
                let _ = connection.set_ex(key, value, seconds).map_err(|e| { format!("Failed to set KV: {} -> {}. Error: {}", key, value, e) })?;
                return Ok(format!("Successfully set: {}:{} with TTL = {}", key, value, seconds));
            },
            CliCommands::TTL => {
                let key = self.arguments.get(0).unwrap();
                let ttl: u64 = connection.ttl(key).map_err(|e| { format!("Failed to retrieve TTL of {}. Error: {}", key, e) })?;
                return Ok(format!("TTL of {} ===> {}", key, ttl));
            },
            CliCommands::QUIT => {
                println!("Quitting app...");
                std::process::exit(0);
            }
        }
    }
}

