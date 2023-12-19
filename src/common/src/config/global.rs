use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// global job config.
#[derive(Serialize, Deserialize, Debug)]
pub struct JobConfig {
    reader: ReaderConfig,
    writer: WriterConfig,
    channel: ChannelConfig,
    setting: SettingConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReaderConfig {
    name: String,

    #[serde(default)]
    database: Vec<String>,

    #[serde(default)]
    table: Vec<String>,
    connection: HashMap<String, String>,
}

/// global writer config.
#[derive(Serialize, Deserialize, Debug)]
pub struct WriterConfig {
    name: String,
    #[serde(default = "default_database_pattern")]
    database_pattern: String,
    connection: HashMap<String, String>,
}

fn default_database_pattern() -> String {
    "_datatrans_{}".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelConfig {
    name: String,
    capacity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingConfig {
    speed: SpeedConfig,
    error: ErrorConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeedConfig {
    num_limit: u32,
    byte_limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorConfig {
    num_limit: u32,
    rate_limit: f32,
}

#[cfg(test)]
mod tests {
    use super::JobConfig;

    #[test]
    fn test_serde() {
        let json = r#"
        {
            "reader": {
                "name": "rdbmsreader",
                "connection": {
                    "url": "",
                    "username": "",
                    "password": ""
                }
            },
            "writer": {
                "name": "dependentList",
                "connection": {
                    "url": "",
                    "username": "",
                    "password": ""
                }
            },
            "channel": {
                "name": "",
                "capacity": 1000
            },
            "setting": {
                "speed": {
                    "num_limit": 112121,
                    "byte_limit": 3424324
                },
                "error": {
                    "num_limit": 33,
                    "rate_limit": 0.01
                }
            }
        }
        "#;
        let job_config = serde_json::from_str::<JobConfig>(json).unwrap();
        println!("{:?}", job_config);
    }
}
