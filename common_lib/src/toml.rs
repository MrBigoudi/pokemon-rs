use std::path::Path;

use log::error;

use super::debug::ErrorCode;

pub struct Toml;

impl Toml {
    pub async fn get_toml(path: &Path) -> Result<toml::Table, ErrorCode> {
        let toml_content = match crate::io::load_string(path).await {
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
}
