use std::str::FromStr;

use core_lib::utils::debug::ErrorCode;
use log::error;

#[derive(Debug)]
pub enum Name {
    Bulbasaur,
    Ivysaur,
    Venusaur,
    //TODO: add more
}

impl FromStr for Name {
    type Err = ErrorCode;

    fn from_str(name: &str) -> Result<Self, ErrorCode> {
        match name {
            "Bulbasaur" => Ok(Name::Bulbasaur),
            "Ivysaur" => Ok(Name::Ivysaur),
            "Venusaur" => Ok(Name::Venusaur),
            _ => {
                error!("The name {} is not a valid pokemon name", name);
                Err(ErrorCode::BadValue)
            }
        }
    }
}
