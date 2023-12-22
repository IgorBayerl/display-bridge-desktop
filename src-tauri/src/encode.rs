// encode.rs
//
// The encoding module handles the encoding of raw screen frame data. It is designed
// to work with the FFmpeg library to encode the captured frames into a desired format
// (e.g., H.264 for video streaming or recording).
//
// Functions:
// - `encode_frame`: Encodes raw frame data using FFmpeg. This function takes the raw
//   data and writes it to FFmpeg's stdin, which is then encoded in real-time.
//
// This module plays a crucial role in preparing the raw data for either streaming
// or saving as a video file. It ensures the data is in a suitable format for
// transmission or storage.

use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use tungstenite::Message;
use std::io::{Error as IoError, ErrorKind};

pub fn start_ffmpeg_encoder(width: u32, height: u32, output: &str) -> Result<std::process::Child, Box<dyn Error>> {
    let child = Command::new("ffmpeg")
        .args(&[
            "-f", "rawvideo",
            "-pix_fmt", "bgra",
            "-s", &format!("{}x{}", width, height),
            "-i", "-",
            "-vcodec", "libx264",
            "-pix_fmt", "yuv420p",
            "-y", output
        ])
        .stdin(Stdio::piped())
        .spawn()?;
    
    Ok(child)
}

pub fn encode_frame(stdin: &mut std::process::ChildStdin, frame_data: &[u8]) -> Result<(), Box<dyn Error>> {
    stdin.write_all(frame_data)?;
    Ok(())
}



// pub async fn encode_frame_websocket(socket: &mut WebSocketStream<TcpStream>, frame: &[u8]) -> Result<(), IoError> {
//     socket.send(Message::Binary(frame.to_vec())).await
//         .map_err(|e| IoError::new(ErrorKind::Other, e))
// }
