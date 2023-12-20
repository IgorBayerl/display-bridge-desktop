import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [captureMsg, setCaptureMsg] = useState("");

  async function captureScreen() {
    try {
      // Invoke the `start_recording_cmd` command
      const msg = await invoke("start_recording_cmd");
      setCaptureMsg(msg as string);
    } catch (error) {
      console.error("Error capturing screen:", error);
      setCaptureMsg("Error capturing screen.");
    }
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <div>
      <button onClick={captureScreen}>Start Recording</button>
      <button onClick={() => invoke("stop_recording_cmd")}>Stop Recording</button>

        <p>{captureMsg}</p>
      </div>
    </div>
  );
}

export default App;
