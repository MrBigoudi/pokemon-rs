use std::sync::Arc;

use common_lib::debug::ErrorCode;
use log::error;

use crate::wgpu_context::state::State;

static mut GLOBAL_WGPU_STATE: Option<Arc<State>> = None;

pub fn set_global_wgpu_state(state: Arc<State>) -> Result<(), ErrorCode> {
    match unsafe { GLOBAL_WGPU_STATE.clone() } {
        Some(_) => {
            error!("Failed to set the global wgpu state; it is already set");
            Err(ErrorCode::AlreadyInitialized)
        }
        None => {
            unsafe { GLOBAL_WGPU_STATE = Some(state) };
            Ok(())
        }
    }
}

pub fn get_global_wgpu_state() -> Result<Arc<State>, ErrorCode> {
    let state = unsafe { GLOBAL_WGPU_STATE.clone() };
    match state {
        Some(state) => Ok(state),
        None => {
            error!("Failed to get the global wgpu state: it is not initialized");
            Err(ErrorCode::NotInitialized)
        }
    }
}
