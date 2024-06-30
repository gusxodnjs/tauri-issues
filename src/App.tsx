import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  async function child() {
   await invoke('child', { label: "child", url: "https://v2.tauri.app" })
  }

  return (
    <div>
      <button onClick={child}>add child</button>
    </div>
  );
}

export default App;
