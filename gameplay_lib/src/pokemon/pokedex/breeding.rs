use std::str::FromStr;

use core_lib::utils::{debug::ErrorCode, toml::Toml};
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

#[derive(Debug)]
pub struct BreedingAttributes {
    pub egg_group_1: EggGroup,
    pub egg_group_2: Option<EggGroup>,
    pub hatch_time: u8,
    pub gender_ratio: GenderRatio,
}

impl BreedingAttributes {
    fn get_egg_groups(toml: &toml::Table) -> Result<(EggGroup, Option<EggGroup>), ErrorCode> {
        let egg_group_names = Toml::get_string_list(toml, "egg_groups")?;
        let egg_groups: Vec<EggGroup> = egg_group_names
            .iter()
            .map(|name| EggGroup::from_str(name))
            .collect::<Result<Vec<EggGroup>, ErrorCode>>()?;
        match egg_groups.as_slice() {
            [group_1] => Ok((*group_1, None)),
            [group_1, group_2] => Ok((*group_1, Some(*group_2))),
            _ => {
                error!("Invalid number of egg groups in the toml");
                Err(ErrorCode::BadValue)
            }
        }
    }

    fn get_hatch_time(toml: &toml::Table) -> Result<u8, ErrorCode> {
        Toml::get_u8(toml, "hatch_time")
    }

    fn get_gender_ratio(toml: &toml::Table) -> Result<GenderRatio, ErrorCode> {
        let gender_ratio = Toml::get_table(toml, "gender_ratio")?;
        let male_ratio = Toml::get_f32(gender_ratio, "male")?;
        let female_ratio = Toml::get_f32(gender_ratio, "female")?;
        let neutral_ratio = Toml::get_f32(gender_ratio, "neutral")?;
        Ok(GenderRatio {
            male: male_ratio,
            female: female_ratio,
            neutral: neutral_ratio,
        })
    }

    pub fn from_toml(toml: &toml::Table) -> Result<BreedingAttributes, ErrorCode> {
        let (egg_group_1, egg_group_2) = Self::get_egg_groups(toml)?;
        let hatch_time = Self::get_hatch_time(toml)?;
        let gender_ratio = Self::get_gender_ratio(toml)?;
        Ok(BreedingAttributes {
            egg_group_1,
            egg_group_2,
            hatch_time,
            gender_ratio,
        })
    }
}
