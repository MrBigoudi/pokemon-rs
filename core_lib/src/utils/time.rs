use cfg_if::cfg_if;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

use std::ops::Sub;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant as StdInstant;

pub type Duration = f64;

/// A platform-independent replacement for `Instant`
#[derive(Copy, Clone)]
pub struct Instant {
    #[cfg(not(target_arch = "wasm32"))]
    inner: StdInstant,

    #[cfg(target_arch = "wasm32")]
    inner: f64, // Store time in milliseconds for wasm
}

impl Default for Instant {
    fn default() -> Self {
        Instant::now()
    }
}

impl Instant {
    /// Creates a new `Instant` at the current time
    pub fn now() -> Self {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let performance = window()
                    .expect("No global `window` exists")
                    .performance()
                    .expect("`performance` should be available");

                Instant {
                    inner: performance.now(), // `now()` returns milliseconds as a float
                }
            } else {
                Instant { inner: StdInstant::now() }
            }
        }
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                (self.inner - other.inner) as Duration
            } else {
                (self.inner - other.inner).as_millis() as Duration
            }
        }
    }
}
