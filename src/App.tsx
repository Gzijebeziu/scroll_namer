import { useState } from "react";
import wieclaw from "./assets/wieclaw.svg";
import { invoke } from "@tauri-apps/api/tauri";
import audio from "./assets/audio.wav";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");

  async function start() {
    new Audio(audio).play()
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet"));
  }

  return (
    <div>
      <div className="container">
      <h1>Generator Nazw Zwojów</h1>
      <p>by Gzijebeziu Żebyr-Żyjgolem</p>
        <img style={{ width: "50%", height: "50%", alignSelf: "center" }} src={wieclaw} className="wieclaw" alt="Więcław. Piękny, prawda?" onClick={() => {greet(); start();}}/>
      </div>
      <div className="container">
      <h2>{greetMsg}</h2>
      </div>
    </div>
  );
}

export default App;
