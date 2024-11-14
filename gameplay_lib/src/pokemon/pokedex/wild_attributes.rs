use crate::pokemon::{abilities::Ability, stats::StatType};

use super::breeding::GenderRatio;

pub type EvYield = (StatType, u8);

pub struct WildAttributes {
    pub abilities: Vec<&'static Ability>,
    pub gender_ratio: GenderRatio,
    
    pub catch_rate: u8,

    pub base_friendship: u8,
    pub base_experience_yield: u16,

    pub ev_yield_1: EvYield,
    pub ev_yield_2: Option<EvYield>,
    pub ev_yield_3: Option<EvYield>,
}