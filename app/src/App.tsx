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

  return (
    <div>
      <h1>Mnemos</h1>
      <h2>Installations</h2>
      <ul>
        {installations.map((inst) => (
          <li key={inst.id}>{inst.display_name} — {inst.executable_path}</li>
        ))}
      </ul>
    </div>
  );
}

export default App;