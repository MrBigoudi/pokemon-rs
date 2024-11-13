use cfg_if::cfg_if;
use core_lib::application::{app::Application, parameters::ApplicationParameters};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn init_env_logger() {
    // Create the output logging file
    use std::fs::File;
    use std::io::Write;
    let target = Box::new(File::create("game_output.log").expect("Failed to create the log file"));
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            // Split the message by newlines and write each line separately with a prefix
            let message = format!("{}", record.args()); // Retrieve message as raw string
            let lines: Vec<_> = message.split('\n').collect();

            for (i, line) in lines.into_iter().enumerate() {
                // Format the first line with metadata
                if i == 0 {
                    writeln!(
                        buf,
                        "{}:{} [{}] - {}",
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0),
                        record.level(),
                        line
                    )?;
                // Process each remaining line in `lines`
                } else {
                    writeln!(buf, "    {}", line)?; // Indent continuation lines for clarity
                }
            }
            Ok(())
        })
        .target(env_logger::Target::Pipe(target))
        .init();
}


#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    // Init the logger
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            init_env_logger();
        }
    }

    // Run the app
    let parameters = ApplicationParameters::default();
    if let Err(err) = Application::run(parameters) {
        panic!("Failed to run the application: {:?}", err);
    }
}