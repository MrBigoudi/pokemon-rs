pub mod breeding;
pub mod categories;
pub mod entry;
pub mod experience_group;
pub mod names;
pub mod wild_attributes;

use common_lib::{debug::ErrorCode, toml::Toml};

use entry::PokedexEntry;

use log::error;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub type Id = u16;

#[non_exhaustive]
pub struct Pokedex {
    pub data: HashMap<Id, PokedexEntry>,
}

impl Pokedex {
    async fn get_toml() -> Result<toml::Table, ErrorCode> {
        let mut pokedex_toml_path = PathBuf::from("");
        pokedex_toml_path.push("assets");
        pokedex_toml_path.push("data");
        pokedex_toml_path.push("pokedex");
        pokedex_toml_path.set_extension("toml");

        Toml::get_toml(&pokedex_toml_path).await
    }

    fn new() -> Result<Self, ErrorCode> {
        let toml = match pollster::block_on(Self::get_toml()){
            Ok(toml) => toml,
            Err(err) => {
                error!(
                    "Failed to block on the Pokedex's toml: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };
        let mut data: HashMap<Id, PokedexEntry> = Default::default();

        // 0001 - Bulbasaur
        let bulbasaur_id = 1 as Id;
        let bulbasaur_entry = PokedexEntry::from_toml(&toml, "bulbasaur", bulbasaur_id)?;
        data.insert(bulbasaur_id, bulbasaur_entry);

        Ok(Self { data })
    }
}

static mut GLOBAL_POKEDEX: Option<Arc<Pokedex>> = None;

pub fn get_global_pokedex() -> Result<Arc<Pokedex>, ErrorCode> {
    let state = unsafe { GLOBAL_POKEDEX.clone() };
    match state {
        Some(state) => Ok(state),
        None => {
            let pokedex = match Pokedex::new() {
                Ok(pokedex) => pokedex,
                Err(err) => {
                    error!("Failed to initialize the global pokedex: {:?}", err);
                    return Err(ErrorCode::Unknown);
                }
            };
            unsafe { GLOBAL_POKEDEX = Some(Arc::new(pokedex)) };
            get_global_pokedex()
        }
    }
}
