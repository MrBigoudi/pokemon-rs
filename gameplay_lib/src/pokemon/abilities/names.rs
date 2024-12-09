use std::str::FromStr;

use core_lib::utils::debug::ErrorCode;
use log::error;

#[derive(Debug)]
pub enum Name {
    Overgrow,
    Chlorophyll,
    // TODO: Add more
}

impl FromStr for Name {
    type Err = ErrorCode;

    fn from_str(name: &str) -> Result<Self, ErrorCode> {
        match name {
            "Overgrow" => Ok(Name::Overgrow),
            "Chlorophyll" => Ok(Name::Chlorophyll),
            _ => {
                error!("The name {} is not a valid ability name", name);
                Err(ErrorCode::BadValue)
            }
        }
    }
}
