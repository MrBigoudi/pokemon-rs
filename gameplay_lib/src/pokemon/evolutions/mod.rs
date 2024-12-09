use core_lib::utils::{debug::ErrorCode, toml::Toml};
use log::error;

#[derive(Debug)]
pub enum EvolutionType {
    LevelUp(u8),
}

impl EvolutionType {
    pub fn from_toml(toml: &toml::Table) -> Result<Self, ErrorCode> {
        let evolution_type = Toml::get_string(toml, "type")?;
        match evolution_type.as_str() {
            "LevelUp" => {
                let level = Toml::get_u8(toml, "level")?;
                Ok(Self::LevelUp(level))
            },
            _ => {
                error!("The type {} is not a valid evolution type", evolution_type);
                Err(ErrorCode::BadValue)
            }
        }
    }
}