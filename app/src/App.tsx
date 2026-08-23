import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

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

type Achievement = {
  name: string;
  description: string | null;
  icon_url: string | null;
  unlocked_at: string | null;
};

function App() {
  const [installations, setInstallations] = useState<Installation[]>([]);
  const [achievements, setAchievements] = useState<Achievement[]>([]);
  const [isAdding, setIsAdding] = useState(false);

  useEffect(() => {
    invoke<Installation[]>("get_installations")
      .then((result) => setInstallations(result))
      .catch((err) => console.error("Failed to fetch installations:", err));
  }, []);

  const handleScan = () => {
    invoke<number>("scan_library")
      .then((count) => {
        console.log(`Discovered ${count} new game(s)`);
        return invoke<Installation[]>("get_installations");
      })
      .then((result) => setInstallations(result))
      .catch((err) => console.error("Scan failed:", err));
  };

  const handleAddFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select a game folder",
      });

      if (typeof selected === "string") {
        await invoke("add_watched_folder", { path: selected });
        console.log("Folder added:", selected);
        handleScan();
      }
    } catch (err) {
      console.error("Failed to add folder:", err);
    }
  };

  const handleAddManually = async () => {
    try {
      setIsAdding(true);
      const selected = await open({
        directory: false,
        multiple: false,
        title: "Select the main .exe file",
        filters: [{ name: "Executable", extensions: ["exe"] }],
      });

      if (typeof selected === "string") {
        await invoke("add_installation_manually", { executablePath: selected });
        console.log("Manually added:", selected);
        const result = await invoke<Installation[]>("get_installations");
        setInstallations(result);
      }
    } catch (err) {
      console.error("Failed to add game manually:", err);
    } finally {
      setIsAdding(false);
    }
  };

  const handleViewAchievements = (installationId: string) => {
    invoke<Achievement[]>("get_achievements_for_installation", { installationId })
      .then(setAchievements)
      .catch((err) => console.error("Failed to fetch achievements:", err));
  };

  return (
    <div>
      <h1>Mnemos</h1>
      <h2>Installations</h2>

      <button onClick={handleScan}>Scan Library</button>
      <button onClick={handleAddFolder}>Add Watched Folder</button>
      <button onClick={handleAddManually} disabled={isAdding}>
        {isAdding ? "Adding..." : "Add Game Manually (exe)"}
      </button>

      <ul>
        {installations.map((inst) => (
          <li key={inst.id}>
            {inst.display_name} — {inst.executable_path}
            {inst.manually_linked && " (Manually Added)"}
            <button onClick={() => handleViewAchievements(inst.id)}>View Achievements</button>
          </li>
        ))}
      </ul>

      {achievements.length > 0 && (
        <div>
          <h3>
            {achievements.filter((a) => a.unlocked_at).length} / {achievements.length} achievements
          </h3>
          <ul>
            {achievements.map((a) => (
              <li key={a.name}>
                {a.unlocked_at ? "Unlocked" : "Locked"} {a.name} {a.description && `— ${a.description}`}
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

export default App;