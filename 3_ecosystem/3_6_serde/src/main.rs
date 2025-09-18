use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub request_type: String,
    pub stream: Stream,
    pub gifts: Vec<Gift>,
    pub debug: DebugInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    pub user_id: Uuid,
    pub is_private: bool,
    pub settings: u32,
    pub shard_url: String,
    pub public_tariff: PublicTariff,
    pub private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTariff {
    pub id: u32,
    pub price: u32,
    pub duration: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateTariff {
    pub client_price: u32,
    pub duration: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gift {
    pub id: u32,
    pub price: u32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    pub duration: String,
    pub at: DateTime<Utc>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = r#"
    {
        "type": "success",
        "stream": {
            "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
            "is_private": false,
            "settings": 45345,
            "shard_url": "https://n3.example.com/sapi",
            "public_tariff": {
                "id": 1,
                "price": 100,
                "duration": "1h",
                "description": "test public tariff"
            },
            "private_tariff": {
                "client_price": 250,
                "duration": "1m",
                "description": "test private tariff"
            }
        },
        "gifts": [{
            "id": 1,
            "price": 2,
            "description": "Gift 1"
        }, {
            "id": 2,
            "price": 3,
            "description": "Gift 2"
        }],
        "debug": {
            "duration": "234ms",
            "at": "2019-06-28T08:35:46+00:00"
        }
    }"#;

    // Deserialize JSON
    let request: Request = serde_json::from_str(json_data)?;
    println!("Successfully deserialized JSON");

    // Convert to YAML
    let yaml_output = serde_yaml::to_string(&request)?;
    println!("YAML Output:");
    println!("{}", yaml_output);

    // Convert to TOML
    let toml_output = toml::to_string(&request)?;
    println!("TOML Output:");
    println!("{}", toml_output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deserialization() {
        let json_data = json!({
            "type": "success",
            "stream": {
                "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
                "is_private": false,
                "settings": 45345,
                "shard_url": "https://n3.example.com/sapi",
                "public_tariff": {
                    "id": 1,
                    "price": 100,
                    "duration": "1h",
                    "description": "test public tariff"
                },
                "private_tariff": {
                    "client_price": 250,
                    "duration": "1m",
                    "description": "test private tariff"
                }
            },
            "gifts": [
                {
                    "id": 1,
                    "price": 2,
                    "description": "Gift 1"
                },
                {
                    "id": 2,
                    "price": 3,
                    "description": "Gift 2"
                }
            ],
            "debug": {
                "duration": "234ms",
                "at": "2019-06-28T08:35:46+00:00"
            }
        });

        let json_string = json_data.to_string();
        let result: Result<Request, _> = serde_json::from_str(&json_string);
        
        assert!(result.is_ok(), "Failed to deserialize JSON");
        
        let request = result.unwrap();
        
        // Test basic fields
        assert_eq!(request.request_type, "success");
        assert_eq!(request.stream.user_id.to_string(), "8d234120-0bda-49b2-b7e0-fbd3912f6cbf");
        assert_eq!(request.stream.is_private, false);
        assert_eq!(request.stream.settings, 45345);
        
        // Test nested structures
        assert_eq!(request.stream.public_tariff.price, 100);
        assert_eq!(request.stream.private_tariff.client_price, 250);
        
        // Test arrays
        assert_eq!(request.gifts.len(), 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
        assert_eq!(request.gifts[1].price, 3);
        
        // Test datetime parsing
        let expected_dt = DateTime::parse_from_rfc3339("2019-06-28T08:35:46+00:00")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(request.debug.at, expected_dt);
    }

    #[test]
    fn test_serialization_to_yaml() {
        let json_data = json!({
            "type": "test",
            "stream": {
                "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
                "is_private": true,
                "settings": 123,
                "shard_url": "https://example.com",
                "public_tariff": {
                    "id": 1,
                    "price": 50,
                    "duration": "30m",
                    "description": "test"
                },
                "private_tariff": {
                    "client_price": 100,
                    "duration": "15m",
                    "description": "test"
                }
            },
            "gifts": [{
                "id": 1,
                "price": 5,
                "description": "Test Gift"
            }],
            "debug": {
                "duration": "100ms",
                "at": "2020-01-01T00:00:00+00:00"
            }
        });

        let request: Request = serde_json::from_value(json_data).unwrap();
        let yaml_result = serde_yaml::to_string(&request);
        
        assert!(yaml_result.is_ok(), "Failed to serialize to YAML");
        
        let yaml_str = yaml_result.unwrap();
        assert!(yaml_str.contains("type: test"));
        assert!(yaml_str.contains("user_id: 8d234120-0bda-49b2-b7e0-fbd3912f6cbf"));
    }

    #[test]
    fn test_serialization_to_toml() {
        let json_data = json!({
            "type": "test",
            "stream": {
                "user_id": "8d234120-0bda-49b2-b7e0-fbd3912f6cbf",
                "is_private": true,
                "settings": 123,
                "shard_url": "https://example.com",
                "public_tariff": {
                    "id": 1,
                    "price": 50,
                    "duration": "30m",
                    "description": "test"
                },
                "private_tariff": {
                    "client_price": 100,
                    "duration": "15m",
                    "description": "test"
                }
            },
            "gifts": [{
                "id": 1,
                "price": 5,
                "description": "Test Gift"
            }],
            "debug": {
                "duration": "100ms",
                "at": "2020-01-01T00:00:00+00:00"
            }
        });

        let request: Request = serde_json::from_value(json_data).unwrap();
        let toml_result = toml::to_string(&request);
        
        assert!(toml_result.is_ok(), "Failed to serialize to TOML");
        
        let toml_str = toml_result.unwrap();
        assert!(toml_str.contains("type = \"test\""));
        assert!(toml_str.contains("user_id = \"8d234120-0bda-49b2-b7e0-fbd3912f6cbf\""));
    }

}