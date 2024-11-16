pub mod breeding;
pub mod categories;
pub mod entry;
pub mod experience_group;
pub mod names;
pub mod wild_attributes;

use entry::PokedexEntry;

#[cfg(not(target_arch = "wasm32"))]
use location_macros::workspace_dir;


use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use crate::core::{debug::ErrorCode, toml::Toml};

pub type Id = u16;

#[non_exhaustive]
pub struct Pokedex {
    pub data: HashMap<Id, PokedexEntry>,
}

pub fn get_pokedex() -> &'static Pokedex {
    static mut GLOBAL_POKEDEX: OnceLock<Pokedex> = OnceLock::new();
    unsafe {
        GLOBAL_POKEDEX.get_or_init(|| {
            Pokedex::new().unwrap_or_else(|err| {
                panic!("Failed to access the global pokedex: {:?}", err);
            })
        })
    }
}

impl Pokedex {
    fn get_toml() -> Result<toml::Table, ErrorCode> {
        #[cfg(not(target_arch = "wasm32"))]
        let mut pokedex_toml_path = PathBuf::from(workspace_dir!());
        #[cfg(target_arch = "wasm32")]
        let mut pokedex_toml_path = PathBuf::from("/");
        pokedex_toml_path.push("assets");
        pokedex_toml_path.push("data");
        pokedex_toml_path.push("pokedex");
        pokedex_toml_path.set_extension("toml");

        Toml::get_toml(&pokedex_toml_path)
    }

    fn new() -> Result<Self, ErrorCode> {
        let toml = Self::get_toml()?;
        let mut data: HashMap<Id, PokedexEntry> = Default::default();

        // 0001 - Bulbasaur
        let bulbasaur_id = 1 as Id;
        let bulbasaur_entry = PokedexEntry::from_toml(&toml, "bulbasaur", bulbasaur_id)?;
        data.insert(bulbasaur_id, bulbasaur_entry);

        Ok(Self { data })
    }
}
