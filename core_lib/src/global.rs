use std::sync::{Arc, Once};

use log::error;

use crate::{application::{utils::debug::ErrorCode, wgpu_context::state::State}, scene::Scene};

static INIT_WGPU_STATE: Once = Once::new();
static mut GLOBAL_WGPU_STATE: Option<Arc<State>> = None;

pub fn set_global_wgpu_state(state: Arc<State>) -> Result<(), ErrorCode> {
    if unsafe { GLOBAL_WGPU_STATE.is_some() } {
        error!("Failed to set the global wgpu state; it is already set");
        return Err(ErrorCode::AlreadyInitialized);
    }
    INIT_WGPU_STATE.call_once(|| {
        unsafe { GLOBAL_WGPU_STATE = Some(state) };
    });
    Ok(())
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
    unsafe {GLOBAL_SCENE = Some(scene) };
    Ok(())
}

pub fn get_global_scene() -> Result<Arc<Scene>, ErrorCode> {
    let scene = unsafe { GLOBAL_SCENE.clone() };
    match scene {
        Some(scene) => Ok(scene),
        None => {
            error!("Failed to get the global scene: it is not initialized");
            Err(ErrorCode::NotInitialized)
        }
    }
}