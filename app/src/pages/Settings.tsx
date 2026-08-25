import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function Settings() {
  const [apiKey, setApiKey] = useState("");
  const [steamId, setSteamId] = useState("");
  const [configured, setConfigured] = useState(false);
  const [status, setStatus] = useState<string | null>(null);

  useEffect(() => {
    invoke<boolean>("get_steam_credentials_status").then(setConfigured);
  }, []);

  const handleSave = async () => {
    try {
      await invoke("save_steam_credentials", { apiKey, steamId });
      setConfigured(true);
      setApiKey("");
      setSteamId("");
      setStatus("Saved.");
    } catch (err) {
      setStatus(String(err));///de
    }
  };

  return (
    <div>
      <h1>Settings</h1>
      <h2>Steam</h2>
      <p>{configured ? "Steam credentials configured" : "Not configured yet"}</p>

      <div style={{ display: "flex", flexDirection: "column", gap: 8, maxWidth: 320 }}>
        <input
          placeholder="Steam Web API key"
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
        />
        <input
          placeholder="SteamID64"
          value={steamId}
          onChange={(e) => setSteamId(e.target.value)}
        />
        <button onClick={handleSave}>Save</button>
        {status && <p>{status}</p>}
      </div>
    </div>
  );
}