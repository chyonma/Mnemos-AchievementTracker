import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core"; 

type Installation = {
  id: string;
  executable_path: string;
  executable_name: string;
  install_directory: string;
  display_name: string;
  known_launcher: string | null;
  steam_app_id: string | null;
  manually_linked: boolean;
};

function App() {
  const [installations, setInstallations] = useState<Installation[]>([]); 

  useEffect(() => {
    invoke<Installation[]>("get_installations")
      .then((result) => setInstallations(result))
      .catch((err) => console.error("Failed to fetch installations:", err));
  }, []);
  const handleScan = () => {
    invoke<number>("scan_library")
      .then((count) => {
        console.log(`Discovered ${count} new game(s)`);
        return invoke<Installation[]>("get_installations"); // refresh the list after scanning
      })
      .then((result) => setInstallations(result))
      .catch((err) => console.error("Scan failed:", err));
  };
  const [folderPath, setFolderPath] = useState("");

  const handleAddFolder = () => {
    invoke("add_watched_folder", { path: folderPath })
    .then(() => setFolderPath(""))
    .catch((err) => console.error("Failed to add folder:", err));
  };
  return (
    <div>
      <h1>Mnemos</h1>
      <h2>Installations</h2>

      <button onClick={handleScan}>Scan Library</button>
      <input
        value={folderPath}
        onChange={(e) => setFolderPath(e.target.value)}
        placeholder="D:\\Games"
      />

      <button onClick={handleAddFolder}>Add Watched Folder</button>
      
      <ul>
        {installations.map((inst) => (
          <li key={inst.id}>{inst.display_name} — {inst.executable_path}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;