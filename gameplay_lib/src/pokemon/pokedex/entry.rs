use std::path::Path;

use crate::pokemon::types::Type;

use super::{breeding::BreedingAttributes, categories::Category, names::Name, wild_attributes::WildAttributes, Id};

pub struct PokedexEntry {
    pub pokedex_number: Id,

    pub name: Name,
    pub category: Category,
    pub description: &'static str,

    pub height: f32,
    pub weight: f32,

    /// A list of (path, description)
    /// Useful if exists in multiple forms + shiny form
    pub sprites: Vec<(&'static Path, &'static str)>,

    pub type_1: Type,
    pub type_2: Option<Type>,

    pub breeding_attributes: BreedingAttributes,
    pub wild_attributes: WildAttributes,
}