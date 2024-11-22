use std::collections::HashMap;

use core_lib::{
    scene::rendering::frame::FrameData,
    utils::{debug::ErrorCode, time::Duration},
    window::key_map::{Key, KeyState},
};
use log::error;

use super::{
    concrete::empty::GameStateEmpty,
    state::{GameState, GameStateType},
};

pub struct GameStatesStack {
    pub stack_of_indices: Vec<GameStateType>,
    pub dict_of_states: HashMap<GameStateType, Box<dyn GameState>>,
}

impl Default for GameStatesStack {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStatesStack {
    /// Initializes a game states stack with an empty state
    fn new() -> Self {
        let state = GameStateEmpty;
        let stack_of_indices = vec![state.get_type()];
        let mut dict_of_states: HashMap<GameStateType, Box<dyn GameState>> = HashMap::new();
        dict_of_states.insert(state.get_type(), Box::new(state));

        Self {
            stack_of_indices,
            dict_of_states,
        }
    }

    /// Adds a state to the state machine
    /// Failes if a state of the same type has already been added
    pub fn add(&mut self, state: Box<dyn GameState>) -> Result<(), ErrorCode> {
        let state_type = state.get_type();
        if self.dict_of_states.contains_key(&state_type) {
            error!("Can't add twice the same type of state in the game state stack");
            return Err(ErrorCode::Duplicate);
        }

        self.dict_of_states.insert(state_type, state);
        Ok(())
    }

    /// Update the states after a change
    fn on_change(&mut self, old_state_type: &GameStateType, new_state_type: &GameStateType) {
        let old_state = self.dict_of_states.get_mut(old_state_type).unwrap();
        old_state.on_exit();
        let new_state = self.dict_of_states.get_mut(new_state_type).unwrap();
        new_state.on_enter();
    }

    /// Push a wanted state to the state stack and update both the old and the current states
    /// Failes if there are no such state in the state machine
    pub fn push(&mut self, new_state_type: GameStateType) -> Result<(), ErrorCode> {
        // Add the state
        if !self.dict_of_states.contains_key(&new_state_type) {
            error!("Can't stack a state which is not present in the state machine");
            return Err(ErrorCode::NotInitialized);
        }

        let old_state_type = *self.stack_of_indices.last().unwrap();
        self.stack_of_indices.push(new_state_type);

        // Update the states
        self.on_change(&old_state_type, &new_state_type);

        Ok(())
    }

    /// Pop a state from the stack and update both the old and the current states
    /// Failes if the stack is empty
    pub fn pop(&mut self) -> Result<(), ErrorCode> {
        match self.stack_of_indices.pop() {
            Some(old_state_type) => {
                let new_state_type = *self.stack_of_indices.last().unwrap();
                self.on_change(&old_state_type, &new_state_type);
                Ok(())
            }
            None => {
                error!("Can't pop from an empty stack");
                Err(ErrorCode::NotInitialized)
            }
        }
    }

    /// Get the state at the top of the stack
    fn get_current_state(&mut self) -> &mut Box<dyn GameState> {
        let current_state_type = self.stack_of_indices.last().unwrap();
        self.dict_of_states.get_mut(current_state_type).unwrap()
    }

    /// The update function runs every frame
    pub fn on_update(&mut self, keys: &HashMap<Key, KeyState>, delta_time: &Duration) -> Result<(), ErrorCode> {
        // Update the current state
        let current_state = self.get_current_state();
        current_state.as_mut().on_update(keys, delta_time);

        // Check for any state on the top of the stack that needs to be swapped or removed
        'swap_states: loop {
            let state = self.get_current_state();
            let state_type = state.get_type();
            if !state.should_be_swapped() {
                break 'swap_states;
            }
            
            self.pop()?;
            let state = self.dict_of_states.get(&state_type).unwrap();
            if state.should_be_removed() {
                self.remove(state_type)?;
            }
        }

        Ok(())
    }

    /// The input handling function runs every frame
    pub fn on_keyboard_input(&mut self, cur_keys: &HashMap<Key, KeyState>, old_keys: &HashMap<Key, KeyState>, new_key: &Key, new_key_state: &KeyState) {
        let current_state = self.get_current_state();
        current_state.as_mut().on_keyboard_input(cur_keys, old_keys, new_key, new_key_state);
    }

    /// The resize function runs on all the states of the machine
    pub fn on_resize(&mut self, new_width: f32, new_height: f32) {
        for (_, state) in self.dict_of_states.iter_mut() {
            state.on_resize(new_width, new_height);
        }
    }

    /// The render function runs every frame
    /// This function calls the render function of all states in the stack in ascending order
    pub fn on_render(&mut self, frame_data: &mut FrameData) -> Result<(), ErrorCode> {
        for state_type in &self.stack_of_indices {
            let state = self.dict_of_states.get_mut(state_type).unwrap();
            if let Err(err) = state.on_render(frame_data) {
                error!("Failed to render the state `{:?}': {:?}", state.get_type(), err);
                return Err(ErrorCode::Unknown);
            }
        }
        Ok(())
    }

    /// Removes all the states from the machine
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Removes one state from the machine
    /// Failes if the state was used in the stack
    /// Failes if the state was not in the machine
    /// Failes if the state is the default state
    pub fn remove(&mut self, state_type: GameStateType) -> Result<(), ErrorCode> {
        if state_type == GameStateType::default() {
            error!("Failed to remove the state `{:?}' from the game states stack: state is the default one", state_type);
            return Err(ErrorCode::BadValue);
        }

        if self.stack_of_indices.contains(&state_type) {
            error!(
                "Failed to remove the state `{:?}' from the game states stack: state is in use",
                state_type
            );
            return Err(ErrorCode::BadValue);
        }

        if !self.dict_of_states.contains_key(&state_type) {
            error!("Failed to remove the state `{:?}' from the game states stack: state is not present", state_type);
            return Err(ErrorCode::BadValue);
        }

        if self.dict_of_states.remove(&state_type).is_none() {
            error!(
                "Failed to remove the state `{:?}' from the game states stack: unknown",
                state_type
            );
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }
}
