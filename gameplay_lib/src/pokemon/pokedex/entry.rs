use std::{path::PathBuf, str::FromStr};

use core_lib::utils::{debug::ErrorCode, toml::Toml};
use log::error;

use crate::pokemon::{stats::Stats, types::Type};

use super::{
    breeding::BreedingAttributes, categories::Category, names::Name,
    wild_attributes::WildAttributes, Id,
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

    pub breeding_attributes: BreedingAttributes,
    pub wild_attributes: WildAttributes,

    pub base_stats: Stats,
}

impl PokedexEntry {
    fn get_pokedex_number(toml: &toml::Table) -> Result<Id, ErrorCode> {
        let pokedex_number = toml.get("id");
        if pokedex_number.is_none() {
            error!("Failed to read the pokedex number: key not found in the toml");
            return Err(ErrorCode::NotFound);
        }
        if let toml::Value::Integer(pokedex_number) = pokedex_number.unwrap() {
            Ok(*pokedex_number as Id)
        } else {
            error!("Failed to read the pokedex number: wrong format in the toml");
            Err(ErrorCode::NotFound)
        }
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
        let breeding_attributes = BreedingAttributes::from_toml(toml)?;
        let wild_attributes = WildAttributes::from_toml(toml)?;
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
            breeding_attributes,
            wild_attributes,
            base_stats,
        })
    }
}
