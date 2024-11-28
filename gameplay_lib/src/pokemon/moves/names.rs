use std::str::FromStr;

use core_lib::utils::debug::ErrorCode;
use log::error;

#[derive(Debug)]
pub enum Name {
    Tackle
    // TODO: Add more
}

impl FromStr for Name {
    type Err = ErrorCode;

    fn from_str(name: &str) -> Result<Self, ErrorCode> {
        match name {
            "Tackle" => Ok(Name::Tackle),
            _ => {
                error!("The name {} is not a valid move name", name);
                Err(ErrorCode::BadValue)
            }
        }
    }
}