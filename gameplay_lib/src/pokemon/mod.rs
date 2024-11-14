use abilities::Ability;

use pokedex::entry::PokedexEntry;

pub mod abilities;
pub mod moves;
pub mod egg;
pub mod pokedex;
pub mod types;
pub mod stats;

pub type Id = usize;

pub struct Pokemon {
    pub pokedex_entry: &'static PokedexEntry,

    pub nickname: String,
    pub id: Id,

    pub ability: &'static Ability,
    pub is_shiny: bool,

    pub friendship: u8,
}