use abilities::Ability;

use moves::Move;
use pokedex::entry::PokedexEntry;

pub mod abilities;
pub mod egg;
pub mod evolutions;
pub mod moves;
pub mod pokedex;
pub mod stats;
pub mod types;

pub type Id = usize;

pub struct Pokemon {
    pub pokedex_entry: &'static PokedexEntry,

    pub nickname: String,
    pub id: Id,

    pub ability: &'static Ability,
    pub is_shiny: bool,

    pub friendship: u8,

    pub move_1: Move,
    pub move_2: Option<Move>,
    pub move_3: Option<Move>,
    pub move_4: Option<Move>,
}
