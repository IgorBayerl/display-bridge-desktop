import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  return (
    <div className="container">
      <h1>Display Bridge Desktop</h1>
      <div>
      <button onClick={() => invoke("start_recording_cmd")}>Start Recording</button>
      <button onClick={() => invoke("stop_recording_cmd")}>Stop Recording</button>
      </div>
    </div>
  );
}

export default App;
