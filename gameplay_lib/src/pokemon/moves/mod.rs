pub enum MoveType {
    Special,
    Physique,
    Status,
}

pub struct Move {
    pub name: &'static str,
    pub move_type: MoveType,

    pub power_points: u8,

    pub power: Option<u8>,
    pub precision: Option<u8>,
}
