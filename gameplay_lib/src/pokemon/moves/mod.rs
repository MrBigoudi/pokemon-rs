use names::Name;

pub mod names;

pub enum MoveType {
    Special,
    Physique,
    Status,
}

pub struct Move {
    pub name: Name,
    pub move_type: MoveType,

    pub power_points: u8,

    pub power: Option<u8>,
    pub precision: Option<u8>,
}
