pub mod breeding;
pub mod categories;
pub mod entry;
pub mod experience_group;
pub mod names;

use core_lib::utils::{debug::ErrorCode, toml::Toml};

use entry::PokedexEntry;

use log::error;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub type Id = u16;

#[derive(Debug)]
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
        let toml = match pollster::block_on(Self::get_toml()) {
            Ok(toml) => toml,
            Err(err) => {
                error!("Failed to block on the Pokedex's toml: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };
        let mut data: HashMap<Id, PokedexEntry> = Default::default();

        let pokedex_array = match toml["pokedex"].as_array(){
            Some(pokedex_array) => pokedex_array,
            None => {
                error!("Failed to read the pokedex from the Pokedex's toml");
                return Err(ErrorCode::NotFound);
            }
        };
        let pokedex_table = match pokedex_array[0].as_table(){
            Some(pokedex_table) => pokedex_table,
            None => {
                error!("Failed to read the pokedex table from the Pokedex's toml");
                return Err(ErrorCode::NotFound);
            }
        };
        let mut id = 0;
        for (name, pokemon_table) in pokedex_table.iter() {
            id+=1;
            let pokemon_table = pokemon_table.as_table().unwrap();
            let pokemon_entry: PokedexEntry = match PokedexEntry::from_toml(pokemon_table) {
                Ok(new_pokemon) => new_pokemon,
                Err(err) => {
                    error!(
                        "Failed to create the pokedex entry for pokemon #{}, {}: {:?}",
                        format!("{:0>4}", id + 1),
                        name,
                        err
                    );
                    return Err(ErrorCode::Unknown);
                }
            };
            data.insert(pokemon_entry.pokedex_number, pokemon_entry);
        }
        Ok(Self{data})

        // for (id, value) in pokedex_array.iter().enumerate() {
        //     let (name, table) = match value.as_table() {
        //         Some(table) => {
        //             let name = table.keys().collect::<Vec<&String>>()[0];
        //             let table = match table[name].as_table() {
        //                 Some(entry) => entry,
        //                 None => {
        //                     error!(
        //                         "Failed to get the pokedex entry for the pokemon entry #{}",
        //                         format!("{:0>4}", id + 1),
        //                     );
        //                     return Err(ErrorCode::Unknown);
        //                 }
        //             };
        //             (name, table)
        //         }
        //         None => {
        //             error!(
        //                 "Failed to get the table for the pokemon entry #{}",
        //                 format!("{:0>4}", id + 1),
        //             );
        //             return Err(ErrorCode::Unknown);
        //         }
        //     };
        //     let new_pokemon = match PokedexEntry::from_toml(table) {
        //         Ok(new_pokemon) => new_pokemon,
        //         Err(err) => {
        //             error!(
        //                 "Failed to create the pokedex entry for pokemon #{}, {}: {:?}",
        //                 format!("{:0>4}", id + 1),
        //                 name,
        //                 err
        //             );
        //             return Err(ErrorCode::Unknown);
        //         }
        //     };
        //     data.insert(new_pokemon.pokedex_number, new_pokemon);
        // }

        // Ok(Self { data })
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
