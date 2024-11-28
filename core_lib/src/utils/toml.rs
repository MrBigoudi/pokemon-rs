use std::path::Path;

use log::error;

use super::debug::ErrorCode;

pub struct Toml;

impl Toml {
    pub async fn get_toml(path: &Path) -> Result<toml::Table, ErrorCode> {
        let toml_content = match super::io::load_string(path).await {
            Ok(content) => content,
            Err(err) => {
                error!("Failed to read the toml file `{:?}': {:?}", path, err);
                return Err(ErrorCode::IO);
            }
        };

        match toml_content.parse::<toml::Table>() {
            Ok(toml) => Ok(toml),
            Err(err) => {
                error!(
                    "Faile to parse the toml file `{:?}' into a toml table: {:?}",
                    &path, err
                );
                Err(ErrorCode::IO)
            }
        }
    }

    pub fn get_u8(toml: &toml::Table, key: &str) -> Result<u8, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!("Failed to read the u8 {}: key not found in the toml", key);
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Integer(value) = value.unwrap() {
            Ok(*value as u8)
        } else {
            error!("Failed to read the u8 {}: wrong format in the toml", key);
            Err(ErrorCode::NotFound)
        }
    }

    pub fn get_u16(toml: &toml::Table, key: &str) -> Result<u16, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!("Failed to read the u16 {}: key not found in the toml", key);
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Integer(value) = value.unwrap() {
            Ok(*value as u16)
        } else {
            error!("Failed to read the u16 {}: wrong format in the toml", key);
            Err(ErrorCode::NotFound)
        }
    }

    pub fn get_u32(toml: &toml::Table, key: &str) -> Result<u32, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!("Failed to read the u32 {}: key not found in the toml", key);
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Integer(value) = value.unwrap() {
            Ok(*value as u32)
        } else {
            error!("Failed to read the u32 {}: wrong format in the toml", key);
            Err(ErrorCode::NotFound)
        }
    }

    pub fn get_f32(toml: &toml::Table, key: &str) -> Result<f32, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!("Failed to read the f32 {}: key not found in the toml", key);
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Float(value) = value.unwrap() {
            Ok(*value as f32)
        } else {
            error!("Failed to read the f32 {}: wrong format in the toml", key);
            Err(ErrorCode::NotFound)
        }
    }

    pub fn get_string(toml: &toml::Table, key: &str) -> Result<String, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!(
                "Failed to read the string {}: key not found in the toml",
                key
            );
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::String(value) = value.unwrap() {
            Ok(value.to_string())
        } else {
            error!(
                "Failed to read the string {}: wrong format in the toml",
                key
            );
            Err(ErrorCode::NotFound)
        }
    }

    pub fn get_string_list(toml: &toml::Table, key: &str) -> Result<Vec<String>, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!(
                "Failed to read the string list {}: key not found in the toml",
                key
            );
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Array(values) = value.unwrap() {
            let values = values
                .iter()
                .filter_map(|s| match s {
                    toml::Value::String(s) => Some(s.to_string()),
                    _ => None,
                })
                .collect();
            Ok(values)
        } else {
            error!(
                "Failed to read the string list {}: wrong format in the toml",
                key
            );
            Err(ErrorCode::NotFound)
        }
    }

    pub fn get_table<'a>(
        toml: &'a toml::Table,
        key: &'a str,
    ) -> Result<&'a toml::Table, ErrorCode> {
        let value = toml.get(key);
        if value.is_none() {
            error!(
                "Failed to read the table {}: key not found in the toml",
                key
            );
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Table(value) = value.unwrap() {
            Ok(value)
        } else {
            error!("Failed to read the table {}: wrong format in the toml", key);
            Err(ErrorCode::NotFound)
        }
    }
}
