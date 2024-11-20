use std::path::Path;

use common_lib::debug::ErrorCode;
use log::error;

use crate::pokemon::{stats::Stats, types::Type};

use super::{
    breeding::BreedingAttributes, categories::Category, names::Name,
    wild_attributes::WildAttributes, Id,
};

pub struct PokedexEntry {
    pub pokedex_number: Id,

    pub name: Name,
    pub category: Category,
    pub description: &'static str,

    pub height: f32,
    pub weight: f32,

    /// A list of (path, description)
    /// Useful if exists in multiple forms
    pub sprites: Vec<(&'static Path, &'static str)>,
    pub shiny_sprites: Vec<(&'static Path, &'static str)>,

    pub type_1: Type,
    pub type_2: Option<Type>,

    pub breeding_attributes: BreedingAttributes,
    pub wild_attributes: WildAttributes,

    pub next_evolution: Option<Id>,
    pub previous_evolution: Option<Id>,

    pub base_stats: Stats,
}

impl PokedexEntry {
    pub fn from_toml(
        _toml: &toml::Table,
        pokemon_name: &str,
        pokemon_id: Id,
    ) -> Result<Self, ErrorCode> {
        // TODO: fill this

        error!(
            "Failed to create the pokedex entry for pokemon #{}: `{}'",
            format!("{:0>4}", pokemon_id),
            pokemon_name
        );
        Err(ErrorCode::Unknown)
    }
}
