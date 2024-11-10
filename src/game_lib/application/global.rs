use std::sync::{Arc, Mutex, OnceLock};

use log::error;

use super::{core::debug::ErrorCode, wgpu_context};

thread_local! {
    pub static GLOBAL_WGPU_STATE: OnceLock<Mutex<Arc<wgpu_context::state::State>>> = const { OnceLock::new() };
}

pub fn get_global_wgpu_state() -> Result<Arc<wgpu_context::state::State>, ErrorCode> {
    // Attempt to get a reference to the Mutex within OnceLock
    GLOBAL_WGPU_STATE.with(|state| {
        match state.get() {
            Some(mutex) => {
                // Try to acquire the lock on the mutex
                match mutex.lock() {
                    Ok(guard) => Ok(guard.clone()),
                    Err(err) => {
                        error!(
                            "Failed to acquire lock for the global wgpu state: {:?}",
                            err
                        );
                        Err(ErrorCode::SyncError)
                    }
                }
            }
            None => {
                error!("Global WGPU state is not initialized");
                Err(ErrorCode::NotInitialized)
            }
        }
    })
}

pub fn set_global_wgpu_state(new_state: Arc<wgpu_context::state::State>) -> Result<(), ErrorCode> {
    // Initialize GLOBAL_WGPU_STATE with a new Mutex-wrapped Arc if it is not already set
    GLOBAL_WGPU_STATE.with(|state| {
        if state.set(Mutex::new(new_state)).is_err() {
            error!("Failed to set the global wgpu state; it is already set");
            Err(ErrorCode::AlreadyInitialized)
        } else {
            Ok(())
        }
    })
}
