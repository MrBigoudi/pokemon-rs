pub mod entry;
pub mod categories;
pub mod names;
pub mod breeding;
pub mod wild_attributes;
pub mod experience_group;

use std::{collections::HashMap, sync::OnceLock};
use entry::PokedexEntry;

pub type Id = u16;

#[non_exhaustive]
pub struct Pokedex{
    pub data: HashMap<Id, PokedexEntry>,
}

impl Pokedex {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

pub fn get_pokedex() -> &'static Pokedex {
    static mut GLOBAL_POKEDEX: OnceLock<Pokedex> = OnceLock::new();
    unsafe { GLOBAL_POKEDEX.get_or_init(|| {
        Pokedex::new()
    }) }
}
