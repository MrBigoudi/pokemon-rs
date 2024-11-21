use cfg_if::cfg_if;
use common_lib::debug::ErrorCode;
use log::error;

use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};
use common_lib::parameters::ApplicationParameters;

pub struct WindowContext;

impl WindowContext {
    #[cfg(not(target_arch = "wasm32"))]
    fn init_window_attributes_desktop(
        parameters: &ApplicationParameters,
        event_loop: &ActiveEventLoop,
    ) -> WindowAttributes {
        let primary_monitor = event_loop.primary_monitor().unwrap();
        let scale_factor = primary_monitor.scale_factor();

        // Desired window size in logical units
        let logical_width = f64::from(parameters.window_width) / scale_factor;
        let logical_height = f64::from(parameters.window_height) / scale_factor;
        use winit::dpi::LogicalSize;
        let window_size = LogicalSize::new(logical_width, logical_height);

        // Monitor size in physical pixels
        let monitor_size: PhysicalSize<u32> = primary_monitor.size();

        // Calculate the top-left position to center the window
        let monitor_width = monitor_size.width as f64;
        let monitor_height = monitor_size.height as f64;
        let pos_x = (monitor_width - logical_width * scale_factor) / 2.0;
        let pos_y = (monitor_height - logical_height * scale_factor) / 2.0;
        use winit::dpi::Position;
        let position = Position::new(Position::Physical((pos_x, pos_y).into()));

        WindowAttributes::default()
            .with_title(&parameters.window_title)
            .with_position(position)
            .with_inner_size(window_size)
    }

    #[cfg(target_arch = "wasm32")]
    fn init_window_attributes_web(
        parameters: &ApplicationParameters,
        _event_loop: &ActiveEventLoop,
    ) -> WindowAttributes {
        use winit::platform::web::WindowAttributesExtWebSys;

        WindowAttributes::default()
            .with_append(true)
            .with_inner_size(PhysicalSize::new(
                parameters.window_width as u32,
                parameters.window_height as u32,
            ))
    }

    pub fn init(
        parameters: &ApplicationParameters,
        event_loop: &ActiveEventLoop,
    ) -> Result<Window, ErrorCode> {
        cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                let window_attributes = Self::init_window_attributes_web(parameters, event_loop);
            } else {
                let window_attributes = Self::init_window_attributes_desktop(parameters, event_loop);
            }
        }

        event_loop.create_window(window_attributes).map_err(|err| {
            error!("Failed to create a winit window: {:?}", err);
            ErrorCode::Winit
        })
    }
}
