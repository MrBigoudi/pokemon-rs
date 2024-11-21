use common_lib::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameStateType {
    Empty,
    // TODO: Add other types of game state
}

/// A game state used in the states stack
/// @see GameStatesStack
pub trait GameState {
    /// Accessor to the type of game state
    fn get_type(&self) -> GameStateType;

    /// Runs every frames if this is the current state
    fn on_update(&mut self, delta_time: Duration);

    /// Runs when leaving the state
    fn on_exit(&mut self);

    /// Runs when entering the state
    fn on_enter(&mut self);

    /// Runs when the window has been resized
    fn on_resize(&mut self, new_width: f32, new_height: f32);

    /// Runs every frame if this is the current state
    fn on_input(&mut self);

    /// Runs every frame if the state is in the stack of states
    fn on_render(&mut self);
}