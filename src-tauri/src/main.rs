// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use scrap::{Capturer, Display};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::process::{Command, Stdio};
use std::io::Write;  // Import the Write trait


// Global flag to control recording state
static IS_RECORDING: AtomicBool = AtomicBool::new(false);


fn capture_and_encode() -> Result<(), Box<dyn Error>> {
    IS_RECORDING.store(true, Ordering::SeqCst);

    let display = Display::primary()?;
    let mut capturer = Capturer::new(display)?;

    let (width, height) = (capturer.width(), capturer.height());
    let output_file = "../output/output.mp4";

    // Set up FFmpeg command to encode video in H.264
    let mut child = Command::new("ffmpeg")
        .args(&[
            "-f", "rawvideo",          // Input format
            "-pix_fmt", "bgra",        // Input pixel format
            "-s", &format!("{}x{}", width, height), // Size of the input frames
            "-i", "-",                 // Input comes from stdin
            "-vcodec", "libx264",      // Use the H.264 codec
            "-pix_fmt", "yuv420p",     // Output pixel format
            "-y", output_file          // Overwrite output file if it exists
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;

    while IS_RECORDING.load(Ordering::SeqCst) {
        match capturer.frame() {
            Ok(frame) => {
                let frame_data = &*frame;

                // Write raw frame data to FFmpeg's stdin
                stdin.write_all(frame_data)?;

                std::thread::sleep(Duration::from_millis(33));
            },
            Err(e) => {
                eprintln!("Frame capture error: {}", e);
                break;
            }
        }
    }

    // Finish writing and close stdin to signal FFmpeg to finalize the video file
    drop(stdin);

    // Wait for FFmpeg to finish
    let _ = child.wait()?;

    Ok(())
}





#[tauri::command]
fn start_recording_cmd() {
    if IS_RECORDING.load(Ordering::SeqCst) {
        eprintln!("Recording is already in progress");
        return;
    }

    std::thread::spawn(|| {
        if let Err(e) = capture_and_encode() {
            eprintln!("Error occurred during recording: {}", e);
        }
    });
}

#[tauri::command]
fn stop_recording_cmd() {
    IS_RECORDING.store(false, Ordering::SeqCst);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_recording_cmd, stop_recording_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}