use common_lib::time::Duration;

use crate::states::state::{GameState, GameStateType};

/// The default game state
pub struct GameStateEmpty;

impl GameState for GameStateEmpty {
    fn get_type(&self) -> GameStateType {
        GameStateType::Empty
    }

    fn on_update(&mut self, _delta_time: Duration) {}

    fn on_exit(&mut self) {}

    fn on_enter(&mut self) {}

    fn on_input(&mut self) {}

    fn on_render(&mut self) {}
    
    fn on_resize(&mut self, _new_width: f32, _new_height: f32) {}
}