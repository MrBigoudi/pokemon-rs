use std::{collections::HashMap, sync::Arc};

use common_lib::debug::ErrorCode;
use common_lib::time::{Duration, Instant};
use log::{error, info};

use common_lib::parameters::ApplicationParameters;

use core_lib::wgpu_context::state::State;
use core_lib::window::{
    init::WindowContext,
    key_map::{Key, KeyState},
};
use winit::{
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

use crate::application::state::ApplicationState;

pub struct Application {
    pub window: Arc<Window>,
    pub wgpu_state: Arc<State>,

    pub last_frame: Instant,
    pub delta_time: Duration,

    pub keys: HashMap<Key, KeyState>,
    #[cfg(not(target_arch = "wasm32"))]
    pub mouse_position: winit::dpi::LogicalPosition<f64>,

    pub default_graphics_pipeline: core_lib::scene::rendering::graphics_pipelines::graphics_default::DefaultGraphicsPipeline,
}

impl Application {
    pub fn new(
        event_loop: &ActiveEventLoop,
        parameters: ApplicationParameters,
    ) -> Result<Application, ErrorCode> {
        info!("Initializing the window...");
        let window = match WindowContext::init(&parameters, event_loop) {
            Ok(window) => Arc::new(window),
            Err(err) => {
                error!("Failed to initialize the application's window: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };

        info!("Initializing the wgpu state...");
        let wgpu_state = match pollster::block_on(State::new(&parameters, window.clone())) {
            Ok(state) => state,
            Err(err) => {
                error!(
                    "Failed to initialize the application's wgpu state: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };
        let wgpu_state = Arc::new(wgpu_state);
        core_lib::wgpu_context::global::set_global_wgpu_state(wgpu_state.clone())?;

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            match web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let element_id = "wasm";
                    let dst = match doc.get_element_by_id(element_id) {
                        Some(dst) => dst,
                        None => {
                            error!("Failed to get the element with id `{}'", element_id);
                            return Some(Err(ErrorCode::Web));
                        }
                    };
                    let canvas = web_sys::Element::from(match window.canvas() {
                        Some(canvas) => canvas,
                        None => {
                            error!("Failed to get the window's canvas");
                            return Some(Err(ErrorCode::Winit));
                        }
                    });
                    if let Err(err) = dst.append_child(&canvas) {
                        error!("Failed to append a child to the document body: {:?}", err);
                        return Some(Err(ErrorCode::Web));
                    };
                    Some(Ok(()))
                }) {
                None => {
                    error!("Failed to append canvas to document body");
                    return Err(ErrorCode::Unknown);
                }
                Some(Err(err)) => {
                    error!("Failed to append canvas to document body: {:?}", err);
                    return Err(ErrorCode::Web);
                }
                _ => (),
            }
        }

        // TODO: Remove this
        // Init the pipelines
        info!("Initializing the pipelines...");
        let default_graphics_pipeline = match pollster::block_on(
            core_lib::scene::rendering::graphics_pipelines::graphics_default::DefaultGraphicsPipeline::new()
        ) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                error!(
                    "Failed to initialize the application's default graphics pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };

        Ok(Application {
            window,
            wgpu_state,
            last_frame: Default::default(),
            delta_time: Default::default(),
            keys: Default::default(),
            #[cfg(not(target_arch = "wasm32"))]
            mouse_position: Default::default(),
            default_graphics_pipeline,
        })
    }

    pub fn run(parameters: ApplicationParameters) -> Result<(), ErrorCode> {
        info!("Initializing the event loop...");
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(err) => {
                error!("Failed to initialize the event loop: {:?}", err);
                return Err(ErrorCode::Winit);
            }
        };
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app_state = ApplicationState::Uninitialized { parameters };
        if let Err(err) = event_loop.run_app(&mut app_state) {
            error!("An error occured during the main event loop: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }
}
