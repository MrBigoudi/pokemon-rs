use std::collections::HashMap;

use core_lib::{
    scene::rendering::frame::FrameData,
    utils::{debug::ErrorCode, time::Duration},
    window::key_map::{Key, KeyState},
};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameStateType {
    #[default]
    Empty,
    // TODO: Remove this
    Test,
    // TODO: Add other types of game state
    OverworldDialog,
}

/// A game state used in the states stack
/// @see GameStatesStack
pub trait GameState {
    /// Tells if the state need to be swapped out
    fn should_be_swapped(&self) -> bool;

    /// Tells if the state should be removed forever
    fn should_be_removed(&self) -> bool;

    /// Accessor to the type of game state
    fn get_type(&self) -> GameStateType;

    /// Runs every frames if this is the current state
    fn on_update(&mut self, keys: &HashMap<Key, KeyState>, delta_time: &Duration);

    /// Runs when leaving the state
    fn on_exit(&mut self);

    /// Runs when entering the state
    fn on_enter(&mut self);

    /// Runs when the window has been resized
    fn on_resize(&mut self, new_width: f32, new_height: f32);

    /// Runs every frame if this is the current state
    fn on_keyboard_input(&mut self, cur_keys: &HashMap<Key, KeyState>, old_keys: &HashMap<Key, KeyState>, new_key: &Key, new_key_state: &KeyState);

    /// Runs every frame if the state is in the stack of states
    fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode>;
}
