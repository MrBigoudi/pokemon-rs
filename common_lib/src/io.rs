use super::debug::ErrorCode;
use cfg_if::cfg_if;
use log::error;

#[cfg(not(target_arch = "wasm32"))]
fn format_path(relative_path: &std::path::Path) -> std::path::PathBuf {
    let root = std::path::PathBuf::from(location_macros::workspace_dir!());
    root.join(relative_path)
}

#[cfg(target_arch = "wasm32")]
fn format_url(relative_path: &std::path::Path) -> Result<reqwest::Url, ErrorCode> {
    let window = match web_sys::window() {
        Some(window) => window,
        None => {
            error!(
                "Failed to get the web sys window when formatting the url `{:?}'",
                relative_path
            );
            return Err(ErrorCode::Unknown);
        }
    };

    let location = window.location();
    let origin = match location.origin() {
        Ok(origin) => origin,
        Err(err) => {
            error!(
                "Failed to get the window location when formatting the url `{:?}': {:?}",
                relative_path, err
            );
            return Err(ErrorCode::Network);
        }
    };

    match reqwest::Url::parse(&format!("{}/{}", origin, relative_path.to_string_lossy())) {
        Ok(url) => Ok(url),
        Err(err) => {
            error!(
                "Failed to parse the url for the given path `{:?}': {:?}",
                relative_path, err
            );
            Err(ErrorCode::Unknown)
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn load_async(url: reqwest::Url) -> Result<reqwest::Response, ErrorCode> {
    match reqwest::get(url.clone()).await {
        Ok(response) => Ok(response),
        Err(err) => {
            error!(
                "Failed to load something with reqwest at `{:?}': {:?}",
                url, err
            );
            Err(ErrorCode::Network)
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn load_string_async(url: reqwest::Url) -> Result<String, ErrorCode> {
    match load_async(url.clone()).await?.text().await {
        Ok(text) => Ok(text),
        Err(err) => {
            error!("Failed to read response text from url `{}': {:?}", url, err);
            Err(ErrorCode::IO)
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn load_bytes_async(url: reqwest::Url) -> Result<Vec<u8>, ErrorCode> {
    match load_async(url.clone()).await?.bytes().await {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(err) => {
            error!(
                "Failed to read response bytes from url `{}': {:?}",
                url, err
            );
            Err(ErrorCode::IO)
        }
    }
}

pub async fn load_string(relative_path: &std::path::Path) -> Result<String, ErrorCode> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(relative_path)?;
            load_string_async(url).await
        } else {
            let absolute_path = format_path(relative_path);
            match std::fs::read_to_string(&absolute_path) {
                Ok(content) => Ok(content),
                Err(err) => {
                    error!("Failed to read the file `{:?}`: {:?}", absolute_path, err);
                    Err(ErrorCode::IO)
                }
            }
        }
    }
}

pub async fn load_bytes(relative_path: &std::path::Path) -> Result<Vec<u8>, ErrorCode> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(relative_path)?;
            load_bytes_async(url).await
        } else {
            let absolute_path = format_path(relative_path);
            match std::fs::read(&absolute_path) {
                Ok(content) => Ok(content),
                Err(err) => {
                    error!("Failed to read the file `{:?}`: {:?}", absolute_path, err);
                    Err(ErrorCode::IO)
                }
            }
        }
    }
}
