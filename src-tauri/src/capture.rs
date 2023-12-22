// capture.rs
//
// This module is responsible for capturing the screen frames. It contains functions
// to initialize the screen capturer and to capture individual frames from the screen.
// The captured frames can then be used for either streaming or recording purposes.
//
// Functions:
// - `initialize_capturer`: Initializes the screen capturer with the primary display.
// - `capture_frame`: Captures a single frame from the initialized capturer.
//
// The module uses the `scrap` crate for capturing screen data. Error handling is
// incorporated to manage potential issues during capture.


use scrap::{Capturer, Display};
use std::error::Error;

pub fn initialize_capturer() -> Result<Capturer, Box<dyn Error>> {
    let display = Display::primary()?;
    Ok(Capturer::new(display)?)
}

pub fn capture_frame(capturer: &mut Capturer) -> Result<Vec<u8>, Box<dyn Error>> {
    match capturer.frame() {
        Ok(frame) => Ok(frame.to_vec()),
        Err(e) => Err(Box::new(e)),
    }
}

