use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;

pub struct ProtonLauncher {
    steam_path: PathBuf,
}

impl ProtonLauncher {
    pub fn new(steam_path: String) -> Self {
        Self {
            steam_path: PathBuf::from(steam_path),
        }
    }

    /// Detect available Proton versions in Steam
    pub fn detect_proton_versions(&self) -> Vec<String> {
        let mut versions = Vec::new();
        
        // Common Proton installation paths
        let proton_paths = vec![
            self.steam_path.join("steamapps/common"),
            self.steam_path.join("compatibilitytools.d"),
        ];

        for base_path in proton_paths {
            if let Ok(entries) = fs::read_dir(&base_path) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.to_lowercase().contains("proton") {
                        versions.push(name);
                    }
                }
            }
        }

        versions.sort();
        versions
    }

    /// Get the path to a specific Proton version
    pub fn get_proton_path(&self, version: &str) -> Option<PathBuf> {
        if version == "Auto-detect" {
            // Try to find the latest Proton version
            let versions = self.detect_proton_versions();
            if let Some(latest) = versions.last() {
                return self.find_proton_executable(latest);
            }
            return None;
        }

        self.find_proton_executable(version)
    }

    fn find_proton_executable(&self, version: &str) -> Option<PathBuf> {
        let proton_paths = vec![
            self.steam_path.join("steamapps/common").join(version).join("proton"),
            self.steam_path.join("compatibilitytools.d").join(version).join("proton"),
        ];

        for path in proton_paths {
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    /// Launch VotV using Proton
    pub fn launch_votv<F>(&self, votv_exe_path: &str, proton_version: &str, mut output_callback: F) -> Result<(), String>
    where
        F: FnMut(String),
    {
        if !Path::new(votv_exe_path).exists() {
            return Err(format!("VotV.exe not found: {}", votv_exe_path));
        }

        let proton_path = self.get_proton_path(proton_version)
            .ok_or_else(|| format!("Proton version '{}' not found", proton_version))?;

        output_callback(format!("Using Proton: {}", proton_path.display()));
        output_callback(format!("Launching: {}", votv_exe_path));

        // Set up Proton environment
        let votv_dir = Path::new(votv_exe_path)
            .parent()
            .ok_or("Invalid VotV.exe path")?;

        // Create a prefix directory for Proton
        let prefix_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gyeet")
            .join("proton_prefix");

        fs::create_dir_all(&prefix_dir)
            .map_err(|e| format!("Failed to create prefix directory: {}", e))?;

        output_callback(format!("Proton prefix: {}", prefix_dir.display()));

        // Launch with Proton
        let mut child = Command::new(&proton_path)
            .arg("run")
            .arg(votv_exe_path)
            .env("STEAM_COMPAT_DATA_PATH", &prefix_dir)
            .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", &self.steam_path)
            .current_dir(votv_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to launch game: {}", e))?;

        output_callback("Game launched! Check the game window.".to_string());

        // Don't wait for the game to exit, let it run in background
        std::thread::spawn(move || {
            let _ = child.wait();
        });

        Ok(())
    }
}

