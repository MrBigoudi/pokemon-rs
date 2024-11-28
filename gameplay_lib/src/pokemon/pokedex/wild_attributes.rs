use std::str::FromStr;

use core_lib::utils::{debug::ErrorCode, toml::Toml};

use crate::pokemon::{abilities::names::Name, stats::Stats};

#[derive(Debug)]
pub struct WildAttributes {
    pub abilities: Vec<Name>,

    pub catch_rate: u8,

    pub base_friendship: u8,
    pub base_experience_yield: u32,

    pub ev_yield: Stats,
}

impl WildAttributes {
    fn get_abilities(toml: &toml::Table) -> Result<Vec<Name>, ErrorCode> {
        let ability_names = Toml::get_string_list(toml, "abilities")?;
        ability_names
            .iter()
            .map(|name| Name::from_str(name))
            .collect()
    }

    fn get_catch_rate(toml: &toml::Table) -> Result<u8, ErrorCode> {
        Toml::get_u8(toml, "catch_rate")
    }

    fn get_base_friendship(toml: &toml::Table) -> Result<u8, ErrorCode> {
        Toml::get_u8(toml, "base_friendship")
    }

    fn get_base_experience_yield(toml: &toml::Table) -> Result<u32, ErrorCode> {
        Toml::get_u32(toml, "base_experience_yield")
    }

    fn get_evs_yield(toml: &toml::Table) -> Result<Stats, ErrorCode> {
        let ev_yield = Toml::get_table(toml, "ev_yield")?;
        Stats::from_toml(ev_yield)
    }

    pub fn from_toml(toml: &toml::Table) -> Result<WildAttributes, ErrorCode> {
        let abilities = Self::get_abilities(toml)?;
        let catch_rate = Self::get_catch_rate(toml)?;
        let base_friendship = Self::get_base_friendship(toml)?;
        let base_experience_yield = Self::get_base_experience_yield(toml)?;
        let ev_yield = Self::get_evs_yield(toml)?;

        Ok(WildAttributes {
            abilities,
            catch_rate,
            base_friendship,
            base_experience_yield,
            ev_yield,
        })
    }
}
