import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [channel, setChannel] = useState("");
  const [status, setStatus] = useState("Esperando...");
  const [isRecording, setIsRecording] = useState(false);

  async function startRecording() {
    if (!channel) {
      setStatus("️ Por favor, escribe un nombre de canal.");
      return;
    }
    
    setStatus(`🔄 Conectando con Twitch: ${channel}...`);
    
    try {
      const result = await invoke<string>("start_recording", { channel });
      setStatus(result);
      setIsRecording(true);
    } catch (error) {
      setStatus(`❌ Error: ${String(error)}`);
    }
  }

  async function stopRecording() {
    setStatus("⏳ Deteniendo grabación...");
    try {
      const result = await invoke<string>("stop_recording");
      setStatus(result);
      setIsRecording(false);
    } catch (error) {
      setStatus(`❌ Error al detener: ${String(error)}`);
    }
  }

  return (
    <main className="container" style={{ textAlign: "center", paddingTop: "50px" }}>
      <h1>🎬 Mi Twitch Recorder</h1>
      <p>Introduce el nombre del canal que quieres grabar:</p>

      <div style={{ marginTop: "20px", display: "flex", justifyContent: "center", gap: "10px" }}>
        <input
          id="channel-input"
          value={channel}
          onChange={(e) => setChannel(e.target.value)}
          placeholder="Ej: auronplay, ibai, etc."
          disabled={isRecording}
          style={{ 
            padding: "10px", 
            fontSize: "16px", 
            borderRadius: "5px", 
            border: "1px solid #ccc", 
            width: "250px" 
          }}
        />
        {!isRecording ? (
          <button 
            onClick={startRecording}
            style={{ 
              padding: "10px 20px", 
              fontSize: "16px", 
              backgroundColor: "#ff0000", 
              color: "white", 
              border: "none", 
              borderRadius: "5px", 
              cursor: "pointer", 
              fontWeight: "bold" 
            }}
          >
            🔴 Grabar
          </button>
        ) : (
          <button 
            onClick={stopRecording}
            style={{ 
              padding: "10px 20px", 
              fontSize: "16px", 
              backgroundColor: "#555555", 
              color: "white", 
              border: "none", 
              borderRadius: "5px", 
              cursor: "pointer", 
              fontWeight: "bold" 
            }}
          >
            ⏹️ Parar
          </button>
        )}
      </div>

      <p style={{ marginTop: "30px", fontSize: "18px", color: "#555" }}>
        {status}
      </p>
    </main>
  );
}

export default App;