use std::{collections::HashMap, sync::Arc};

use core_lib::utils::config::ApplicationParameters;
use core_lib::utils::debug::ErrorCode;
use core_lib::utils::time::{Duration, Instant};
use gameplay_lib::states::states_stack::GameStatesStack;
use log::{error, info};

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
    pub target_frame_time: Duration,

    pub keys: HashMap<Key, KeyState>,
    pub last_keys: HashMap<Key, KeyState>,

    #[cfg(not(target_arch = "wasm32"))]
    pub mouse_position: winit::dpi::LogicalPosition<f64>,

    pub game_states: GameStatesStack,
}

impl Application {
    /// Initializes the winit window
    fn init_window(
        event_loop: &ActiveEventLoop,
        parameters: &ApplicationParameters,
    ) -> Result<Arc<Window>, ErrorCode> {
        match WindowContext::init(parameters, event_loop) {
            Ok(window) => Ok(Arc::new(window)),
            Err(err) => {
                error!("Failed to initialize the application's window: {:?}", err);
                Err(ErrorCode::Unknown)
            }
        }
    }

    /// Initializes the wgpu context
    fn init_state(
        parameters: &ApplicationParameters,
        window: Arc<Window>,
    ) -> Result<Arc<State>, ErrorCode> {
        match pollster::block_on(State::new(parameters, window.clone())) {
            Ok(state) => Ok(Arc::new(state)),
            Err(err) => {
                error!(
                    "Failed to initialize the application's wgpu state: {:?}",
                    err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    /// Initializes the html canvas for wasm
    fn init_canvas(window: Arc<Window>) -> Result<(), ErrorCode> {
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

    /// Initializes the game states
    fn init_game_states() -> Result<GameStatesStack, ErrorCode> {
        let mut game_states = GameStatesStack::default();

        // TODO: add oter states
        if let Err(err) = game_states.add(Box::new(
            gameplay_lib::states::concrete::overworld::GameStateOverworld::default(),
        )) {
            error!("Failed to create the overworld game state: {:?}", err);
            return Err(ErrorCode::Unknown);
        }
        if let Err(err) = game_states.add(Box::new(
            gameplay_lib::states::concrete::overworld_dialog::GameStateOverworldDialog::default(),
        )) {
            error!("Failed to create the overworld dialog game state: {:?}", err);
            return Err(ErrorCode::Unknown);
        }


        // TODO: remove this
        if let Err(err) = game_states.push(gameplay_lib::states::state::GameStateType::Overworld) {
            error!("Failed to push the overworld game state: {:?}", err);
            return Err(ErrorCode::Unknown);
        }
        if let Err(err) = game_states.push(gameplay_lib::states::state::GameStateType::OverworldDialog) {
            error!("Failed to push the overworld dialog game state: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(game_states)
    }

    /// Create the application
    pub fn new(
        event_loop: &ActiveEventLoop,
        parameters: ApplicationParameters,
    ) -> Result<Application, ErrorCode> {
        info!("Initializing the window...");
        let window = Self::init_window(event_loop, &parameters)?;

        info!("Initializing the wgpu state...");
        let wgpu_state = Self::init_state(&parameters, window.clone())?;
        core_lib::wgpu_context::global::set_global_wgpu_state(wgpu_state.clone())?;

        #[cfg(target_arch = "wasm32")]
        {
            info!("Initializing the canvas...");
            Self::init_canvas(window.clone())?;
        }

        info!("Initializing the game states...");
        let game_states = Self::init_game_states()?;

        Ok(Application {
            window,
            wgpu_state,
            last_frame: Default::default(),
            delta_time: Default::default(),
            target_frame_time: (1. / parameters.max_frame_rate as Duration),

            keys: Default::default(),
            last_keys: Default::default(),

            #[cfg(not(target_arch = "wasm32"))]
            mouse_position: Default::default(),
            game_states,
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
