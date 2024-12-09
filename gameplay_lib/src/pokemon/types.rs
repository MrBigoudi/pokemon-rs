use std::str::FromStr;

use core_lib::utils::debug::ErrorCode;
use log::error;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
    Stellar,
    Unknown,
}

impl FromStr for Type {
    type Err = ErrorCode;

    fn from_str(type_name: &str) -> Result<Self, ErrorCode> {
        match type_name {
            "Normal" => Ok(Type::Normal),
            "Fire" => Ok(Type::Fire),
            "Fighting" => Ok(Type::Fighting),
            "Water" => Ok(Type::Water),
            "Flying" => Ok(Type::Flying),
            "Grass" => Ok(Type::Grass),
            "Poison" => Ok(Type::Poison),
            "Electric" => Ok(Type::Electric),
            "Psychic" => Ok(Type::Psychic),
            "Ground" => Ok(Type::Ground),
            "Rock" => Ok(Type::Rock),
            "Ice" => Ok(Type::Ice),
            "Bug" => Ok(Type::Bug),
            "Dragon" => Ok(Type::Dragon),
            "Ghost" => Ok(Type::Ghost),
            "Dark" => Ok(Type::Dark),
            "Steel" => Ok(Type::Steel),
            "Fairy" => Ok(Type::Fairy),
            "Stellar" => Ok(Type::Stellar),
            "Unknown" => Ok(Type::Unknown),
            _ => {
                error!("The type {} is not a valid pokemon type", type_name);
                Err(ErrorCode::BadValue)
            }
        }
    }
}
