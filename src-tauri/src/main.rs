// main.rs
//
// The main entry point of the Tauri application. This file sets up the Tauri
// application, initializes the screen capturer, and manages the commands for
// starting and stopping recording. It uses the modularized functionalities
// provided by the capture, encode, record, and stream modules.

mod capture;
mod encode;
mod record;
mod stream;

use tauri::Builder;
use record::{start_recording_cmd, stop_recording_cmd};
// use stream::{start_streaming_cmd, stop_streaming_cmd};

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_recording_cmd,
            stop_recording_cmd,
            // start_streaming_cmd,
            // stop_streaming_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
