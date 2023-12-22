// stream.rs
//
// This module focuses on the streaming functionality of the application. It provides
// the necessary functions to start and stop streaming the captured screen data.
//
// Functions:
// - `start_streaming_cmd`: Starts the streaming of screen data. This involves capturing
//   screen frames and sending them to a specified server or endpoint in real-time.
// - `stop_streaming_cmd`: Stops the streaming process.
//
// The module should handle the setup and teardown of any network connections or
// protocols required for streaming. It works in conjunction with the capture and
// encode modules to process and transmit the screen data.


use crate::capture::{initialize_capturer, capture_frame};
// use crate::encode::encode_frame_websocket;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use std::sync::Arc;

static IS_STREAMING: AtomicBool = AtomicBool::new(false);

// #[tauri::command]
// pub async fn start_streaming_cmd() -> Result<(), String> {
//     if IS_STREAMING.load(Ordering::SeqCst) {
//         return Err("Streaming is already in progress".into());
//     }

//     IS_STREAMING.store(true, Ordering::SeqCst);
//     tokio::spawn(async {
//         if let Err(e) = start_streaming().await {
//             eprintln!("Error occurred during streaming: {}", e);
//         }
//     });

//     Ok(())
// }

// async fn start_streaming() -> Result<(), String> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await.map_err(|e| e.to_string())?;
//     let capturer = Arc::new(Mutex::new(initialize_capturer().map_err(|e| e.to_string())?));

//     while let Ok((stream, _)) = listener.accept().await {
//         let capturer = capturer.clone();
//         tokio::spawn(async move {
//             let websocket = accept_async(stream).await.expect("Error during WebSocket handshake");
//             if let Ok(mut websocket) = websocket {
//                 let mut capturer = capturer.lock().await;
//                 while IS_STREAMING.load(Ordering::SeqCst) {
//                     let frame = capture_frame(&mut *capturer).unwrap();
//                     encode_frame_websocket(&mut websocket, &frame).await.unwrap(); // Encode and send over WebSocket
//                     tokio::time::sleep(Duration::from_millis(33)).await; // Adjust based on desired frame rate
//                 }
//             }
//         });
//     }
//     Ok(())
// }

#[tauri::command]
pub fn stop_streaming_cmd() {
    IS_STREAMING.store(false, Ordering::SeqCst);
}
