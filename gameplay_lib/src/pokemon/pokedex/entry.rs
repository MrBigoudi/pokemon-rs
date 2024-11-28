use std::{path::PathBuf, str::FromStr};

use core_lib::utils::{debug::ErrorCode, toml::Toml};
use log::error;

use crate::pokemon::{abilities, evolutions::EvolutionType, stats::Stats, types::Type};

use super::{
    breeding::{EggGroup, GenderRatio}, categories::Category, names::Name,
    Id,
};

#[derive(Debug)]
pub struct PokedexEntry {
    pub pokedex_number: Id,

    pub name: Name,
    pub category: Category,
    pub description: String,

    pub height: f32,
    pub weight: f32,

    /// A list of (path, description)
    /// Useful if exists in multiple forms
    pub sprites: Vec<PathBuf>,
    pub shiny_sprites: Vec<PathBuf>,

    pub type_1: Type,
    pub type_2: Option<Type>,
    pub previous_evolution: Option<Id>,
    pub next_evolution: Option<Id>,
    pub evolution_type: Option<EvolutionType>,
    pub base_stats: Stats,

    pub egg_group_1: EggGroup,
    pub egg_group_2: Option<EggGroup>,
    pub hatch_time: u8,
    pub gender_ratio: GenderRatio,
    
    pub abilities: Vec<abilities::names::Name>,
    pub catch_rate: u8,
    pub base_friendship: u8,
    pub base_experience_yield: u32,
    pub ev_yield: Stats,

}

impl PokedexEntry {
    fn get_pokedex_number(toml: &toml::Table) -> Result<Id, ErrorCode> {
        Ok(Toml::get_u16(toml, "id")? as Id)
    }

    fn get_name(toml: &toml::Table) -> Result<Name, ErrorCode> {
        let name = Toml::get_string(toml, "name")?;
        Name::from_str(&name)
    }

    fn get_category(toml: &toml::Table) -> Result<Category, ErrorCode> {
        let category = Toml::get_string(toml, "category")?;
        Category::from_str(&category)
    }

    fn get_description(toml: &toml::Table) -> Result<String, ErrorCode> {
        Toml::get_string(toml, "description")
    }

    fn get_height(toml: &toml::Table) -> Result<f32, ErrorCode> {
        Toml::get_f32(toml, "height")
    }

    fn get_weight(toml: &toml::Table) -> Result<f32, ErrorCode> {
        Toml::get_f32(toml, "weight")
    }

    fn get_sprites(toml: &toml::Table) -> Result<Vec<PathBuf>, ErrorCode> {
        let sprite_names = Toml::get_string_list(toml, "sprites")?;
        Ok(sprite_names.iter().map(PathBuf::from).collect())
    }

    fn get_shiny_sprites(toml: &toml::Table) -> Result<Vec<PathBuf>, ErrorCode> {
        let sprite_names = Toml::get_string_list(toml, "shiny_sprites")?;
        Ok(sprite_names.iter().map(PathBuf::from).collect())
    }

    fn get_types(toml: &toml::Table) -> Result<(Type, Option<Type>), ErrorCode> {
        let type_names = Toml::get_string_list(toml, "types")?;
        let types: Vec<Type> = type_names
            .iter()
            .map(|name| Type::from_str(name))
            .collect::<Result<Vec<Type>, ErrorCode>>()?;
        match types.as_slice() {
            [type_1] => Ok((*type_1, None)),
            [type_1, type_2] => Ok((*type_1, Some(*type_2))),
            _ => {
                error!("Invalid number of types in the toml");
                Err(ErrorCode::BadValue)
            }
        }
    }

    fn get_base_stats(toml: &toml::Table) -> Result<Stats, ErrorCode> {
        let base_stats = Toml::get_table(toml, "base_stats")?;
        Stats::from_toml(base_stats)
    }

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

    fn get_abilities(toml: &toml::Table) -> Result<Vec<abilities::names::Name>, ErrorCode> {
        let ability_names = Toml::get_string_list(toml, "abilities")?;
        ability_names
            .iter()
            .map(|name| abilities::names::Name::from_str(name))
            .collect()
    }

    fn get_catch_rate(toml: &toml::Table) -> Result<u8, ErrorCode> {
        Toml::get_u8(toml, "catch_rate")
    }

    fn get_base_friendship(toml: &toml::Table) -> Result<u8, ErrorCode> {
        Toml::get_u8(toml, "base_friendship")
    }

    fn get_base_experience_yield(toml: &toml::Table) -> Result<u32, ErrorCode> {
        Toml::get_u32(toml, "base_experience_yield")
    }

    fn get_evs_yield(toml: &toml::Table) -> Result<Stats, ErrorCode> {
        let ev_yield = Toml::get_table(toml, "ev_yield")?;
        Stats::from_toml(ev_yield)
    }

    fn get_previous_evolution(toml: &toml::Table) -> Result<Option<Id>, ErrorCode> {
        let id = Toml::get_u16(toml, "previous_evolution")? as Id;
        if id == 0 {
            Ok(None)
        } else {
            Ok(Some(id))
        }
    }

    fn get_next_evolution(toml: &toml::Table) -> Result<Option<Id>, ErrorCode> {
        let id = Toml::get_u16(toml, "next_evolution")? as Id;
        if id == 0 {
            Ok(None)
        } else {
            Ok(Some(id))
        }
    }

    fn get_evolution_type(toml: &toml::Table) -> Result<Option<EvolutionType>, ErrorCode> {
        let evolution_type_table = Toml::get_table(toml, "evolution_type")?;
        if evolution_type_table.is_empty() {
            Ok(None)
        } else {
            Ok(Some(EvolutionType::from_toml(evolution_type_table)?))
        }
    }

    pub fn from_toml(toml: &toml::Table) -> Result<Self, ErrorCode> {
        let pokedex_number = Self::get_pokedex_number(toml)?;
        let name = Self::get_name(toml)?;
        let category = Self::get_category(toml)?;
        let description = Self::get_description(toml)?;
        let height = Self::get_height(toml)?;
        let weight = Self::get_weight(toml)?;
        let sprites = Self::get_sprites(toml)?;
        let shiny_sprites = Self::get_shiny_sprites(toml)?;
        let (type_1, type_2) = Self::get_types(toml)?;
        let previous_evolution = Self::get_previous_evolution(toml)?;
        let next_evolution = Self::get_next_evolution(toml)?;
        let evolution_type = Self::get_evolution_type(toml)?;
        let (egg_group_1, egg_group_2) = Self::get_egg_groups(toml)?;
        let hatch_time = Self::get_hatch_time(toml)?;
        let gender_ratio = Self::get_gender_ratio(toml)?;
        let abilities = Self::get_abilities(toml)?;
        let catch_rate = Self::get_catch_rate(toml)?;
        let base_friendship = Self::get_base_friendship(toml)?;
        let base_experience_yield = Self::get_base_experience_yield(toml)?;
        let ev_yield = Self::get_evs_yield(toml)?;
        let base_stats = Self::get_base_stats(toml)?;

        Ok(Self {
            pokedex_number,
            name,
            category,
            description,
            height,
            weight,
            sprites,
            shiny_sprites,
            type_1,
            type_2,
            previous_evolution,
            next_evolution,
            evolution_type,
            base_stats,
            egg_group_1,
            egg_group_2,
            hatch_time,
            gender_ratio,
            abilities,
            catch_rate,
            base_friendship,
            base_experience_yield,
            ev_yield,
        })
    }
}
