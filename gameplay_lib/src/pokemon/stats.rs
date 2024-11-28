use core_lib::utils::{debug::ErrorCode, toml::Toml};

pub enum StatType {
    Hp,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

#[derive(Debug)]
pub struct Stats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}

impl Stats {
    pub fn from_toml(toml: &toml::Table) -> Result<Self, ErrorCode> {
        let hp = Toml::get_u16(toml, "hp")?;
        let attack = Toml::get_u16(toml, "attack")?;
        let defense = Toml::get_u16(toml, "defense")?;
        let special_attack = Toml::get_u16(toml, "special_attack")?;
        let special_defense = Toml::get_u16(toml, "special_defense")?;
        let speed = Toml::get_u16(toml, "speed")?;

        Ok(Self {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        })
    }
}
