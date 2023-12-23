// record.rs
//
// The recording module is dedicated to recording the screen data into a video file.
// It provides commands to start and stop the recording process.
//
// Functions:
// - `start_recording_cmd`: Begins the recording process. This involves capturing the
//   screen frames, encoding them, and then writing the encoded data to a video file.
// - `stop_recording_cmd`: Ends the recording process and ensures the video file is
//   properly closed and saved.
//
// This module uses functionalities from the capture and encode modules to acquire and
// process the screen data. It manages the state of the recording process using an
// AtomicBool to ensure thread-safe operations.

use crate::capture::{initialize_capturer, capture_frame};
use crate::encode::{start_ffmpeg_encoder, encode_frame};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static IS_RECORDING: AtomicBool = AtomicBool::new(false);

fn milliseconds_per_frame(fps: u32) -> u64 {
    1000 / fps as u64
}

#[tauri::command]
pub fn start_recording_cmd() -> Result<(), String> {
    if IS_RECORDING.load(Ordering::SeqCst) {
        return Err("Recording is already in progress".into());
    }

    IS_RECORDING.store(true, Ordering::SeqCst);

    std::thread::spawn(|| {
        if let Err(e) = record_screen() {
            eprintln!("Error occurred during recording: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_recording_cmd() {
    IS_RECORDING.store(false, Ordering::SeqCst);
}

#[tauri::command]
pub fn is_recording_cmd() -> bool {
    IS_RECORDING.load(Ordering::SeqCst)
}


fn record_screen() -> Result<(), String> {
    let mut capturer = initialize_capturer().map_err(|e| e.to_string())?;
    let width = capturer.width() as u32;
    let height = capturer.height() as u32;
    let mut ffmpeg = start_ffmpeg_encoder(width, height, "../output/output.mp4").map_err(|e| e.to_string())?;
    let stdin = ffmpeg.stdin.as_mut().ok_or("Failed to open stdin")?;

    let ms_to_wait = milliseconds_per_frame(30);
    while IS_RECORDING.load(Ordering::SeqCst) {
        let frame_data = capture_frame(&mut capturer).map_err(|e| e.to_string())?;
        encode_frame(stdin, &frame_data).map_err(|e| e.to_string())?;
        std::thread::sleep(Duration::from_millis(ms_to_wait));
    }

    let _ = ffmpeg.wait().map_err(|e| e.to_string())?;
    Ok(())
}


