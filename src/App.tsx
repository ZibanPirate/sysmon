import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  return (
    <div className="container">
      <h1>System Monitor</h1>

      <button onClick={() => invoke("close")}>Close</button>
    </div>
  );
}

export default App;
