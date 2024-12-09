use std::str::FromStr;

use core_lib::utils::debug::ErrorCode;
use log::error;

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Neutral,
}

#[derive(Debug)]
pub struct GenderRatio {
    pub male: f32,
    pub female: f32,
    pub neutral: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum EggGroup {
    Monster,
    HumanLike,
    Water1,
    Water2,
    Water3,
    Bug,
    Mineral,
    Flying,
    Amorphous,
    Field,
    Fairy,
    Ditto,
    Grass,
    Dragon,
    NoEggsDiscovered,
}

impl FromStr for EggGroup {
    fn from_str(egg_group: &str) -> Result<Self, ErrorCode> {
        match egg_group {
            "Monster" => Ok(EggGroup::Monster),
            "HumanLike" => Ok(EggGroup::HumanLike),
            "Water1" => Ok(EggGroup::Water1),
            "Water2" => Ok(EggGroup::Water2),
            "Water3" => Ok(EggGroup::Water3),
            "Bug" => Ok(EggGroup::Bug),
            "Mineral" => Ok(EggGroup::Mineral),
            "Flying" => Ok(EggGroup::Flying),
            "Amorphous" => Ok(EggGroup::Amorphous),
            "Field" => Ok(EggGroup::Field),
            "Fairy" => Ok(EggGroup::Fairy),
            "Ditto" => Ok(EggGroup::Ditto),
            "Grass" => Ok(EggGroup::Grass),
            "Dragon" => Ok(EggGroup::Dragon),
            "NoEggsDiscovered" => Ok(EggGroup::NoEggsDiscovered),
            _ => {
                error!(
                    "The egg group {} is not a valid pokemon egg group",
                    egg_group
                );
                Err(ErrorCode::BadValue)
            }
        }
    }

    type Err = ErrorCode;
}

pub const NB_STEPS_PER_EGG_CYCLE: u8 = 0xFF;