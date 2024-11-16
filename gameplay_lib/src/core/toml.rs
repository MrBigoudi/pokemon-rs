use std::path::Path;

use log::error;

use super::debug::ErrorCode;

pub struct Toml;

impl Toml {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_toml(path: &Path) -> Result<toml::Table, ErrorCode> {
        let toml_content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                error!(
                    "Failed to read the toml file `{:?}': {:?}",
                    path, err
                );
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

    #[cfg(target_arch = "wasm32")]
    pub fn get_toml(url: &Path) -> Result<toml::Table, ErrorCode> {
        let url: &str = url.as_os_str().to_str().unwrap();
        let toml_content = match pollster::block_on(Self::fetch_toml(url)) {
            Ok(content) => content,
            Err(err) => {
                error!(
                    "Failed to fetch the toml file `{:?}': {:?}",
                    url, err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        match toml_content.parse::<toml::Table>() {
            Ok(toml) => Ok(toml),
            Err(err) => {
                error!(
                    "Faile to parse the file `{:?}' into a toml table: {:?}",
                    &url, err
                );
                Err(ErrorCode::IO)
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn fetch_toml(url: &str) -> Result<String, ErrorCode> {
        let response = reqwest::get(url).await.map_err(|err| {
            error!("Failed to fetch the toml file `{}`: {:?}", url, err);
            ErrorCode::Network
        })?;

        if response.status().is_success() {
            let text = response.text().await.map_err(|err| {
                error!("Failed to read toml response text from `{}`: {:?}", url, err);
                ErrorCode::IO
            })?;
            Ok(text)
        } else {
            error!("Failed to fetch toml `{}`: HTTP {}", url, response.status());
            Err(ErrorCode::Network)
        }
    }
}