pub mod breeding;
pub mod categories;
pub mod entry;
pub mod experience_group;
pub mod names;
pub mod wild_attributes;

use entry::PokedexEntry;
use location_macros::workspace_dir;
use log::error;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use crate::core::debug::ErrorCode;

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
        let mut pokedex_toml_path = PathBuf::from(workspace_dir!());
        pokedex_toml_path.push("assets");
        pokedex_toml_path.push("data");
        pokedex_toml_path.push("pokedex");
        pokedex_toml_path.set_extension("toml");

        let pokedex_toml = match std::fs::read_to_string(&pokedex_toml_path) {
            Ok(content) => content,
            Err(err) => {
                error!(
                    "Failed to read the pokedex toml file `{:?}': {:?}",
                    pokedex_toml_path, err
                );
                return Err(ErrorCode::I0);
            }
        };

        match pokedex_toml.parse::<toml::Table>() {
            Ok(toml) => Ok(toml),
            Err(err) => {
                error!(
                    "Faile to parse the file `{:?}' into a toml table: {:?}",
                    &pokedex_toml_path, err
                );
                Err(ErrorCode::I0)
            }
        }
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
