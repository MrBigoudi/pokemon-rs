use std::sync::Arc;

use common_lib::debug::ErrorCode;
use log::error;

use crate::{application::wgpu_context::state::State, scene::Scene};

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

static mut GLOBAL_SCENE: Option<Arc<Scene>> = None;

pub fn set_global_scene(scene: Arc<Scene>) -> Result<(), ErrorCode> {
    unsafe { GLOBAL_SCENE = Some(scene) };
    Ok(())
}

pub fn get_global_scene() -> Result<Arc<Scene>, ErrorCode> {
    match unsafe { GLOBAL_SCENE.clone() } {
        Some(scene) => Ok(scene),
        None => {
            error!("Failed to get the global scene: it is not initialized");
            Err(ErrorCode::NotInitialized)
        }
    }
}
