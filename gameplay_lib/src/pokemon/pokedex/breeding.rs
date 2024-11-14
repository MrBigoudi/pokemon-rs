#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Neutral,
}

pub struct GenderRatio {
    pub male: f32,
    pub female: f32,
    pub neutral: f32,
}

#[derive(Debug)]
pub enum EggGroup {
    Monster,
    HumanLike,
    Water1,
    Water2,
    Water3,
    Bug,
    Mineral,
    Flying,
    Amorphous,
    Field,
    Fairy,
    Ditto,
    Grass,
    Dragon,
    NoEggsDiscovered,
}

pub const NB_STEPS_PER_EGG_CYCLE: u8 = 0xFF;

pub struct BreedingAttributes {
    pub egg_group_1: EggGroup,
    pub egg_group_2: Option<EggGroup>,
    pub hatch_time: Option<u8>,
}