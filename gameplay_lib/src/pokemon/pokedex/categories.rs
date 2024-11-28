use std::str::FromStr;

use core_lib::utils::debug::ErrorCode;
use log::error;

#[derive(Debug)]
pub enum Category {
    Seed,
}

impl FromStr for Category {
    fn from_str(category: &str) -> Result<Self, ErrorCode> {
        match category {
            "Seed" => Ok(Category::Seed),
            _ => {
                error!("The category {} is not a valid pokemon category", category);
                Err(ErrorCode::BadValue)
            }
        }
    }

    type Err = ErrorCode;
}
