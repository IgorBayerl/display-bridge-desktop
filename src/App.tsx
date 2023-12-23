import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { useEffect, useState } from "react";

function App() {
  const [isRecording, setIsRecording] = useState(false);

  const checkRecordingStatus = () => {
    invoke("is_recording_cmd").then((result) => {
      setIsRecording(result as boolean);
    });
  };
  

  useEffect(() => {
    const interval = setInterval(checkRecordingStatus, 1000); 
    return () => clearInterval(interval);
  }, []);



  return (
    <div className="container">
      <h1>Display Bridge Desktop</h1>
      <div>
      <button onClick={() => invoke("start_recording_cmd")}>Start Recording</button>
      <button onClick={() => invoke("stop_recording_cmd")}>Stop Recording</button>
      </div>
      <br />
      <br />
      <div>
        Recording Status: {isRecording ? "Recording..." : "Not Recording"}
      </div>
    </div>
  );
}

export default App;
