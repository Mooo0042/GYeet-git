use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

const PATCH_MANIFEST_URL: &str = "https://votv.dev/patcher_assets/patch_manifest.json";
const INSTALL_CATALOG_URL: &str = "https://votv.dev/patcher_assets/index_manifest.json";
const STORE_URL: &str = "https://votv.dev/patcher_assets/256-1024-4096-store";
const DESYNC_URL: &str =
    "https://github.com/folbricht/desync/releases/download/v0.9.6/desync_0.9.6_linux_amd64.tar.gz";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersion {
    pub name: String,
    pub hash: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
struct PatchManifest {
    latest: String,
    #[serde(rename = "fileHashMap")]
    file_hash_map: HashMap<String, String>,
    patches: HashMap<String, PatchInfo>,
}

#[derive(Debug, Deserialize)]
struct PatchInfo {
    url: String,
    sha256: String,
}

pub struct Patcher {}

impl Patcher {
    pub fn new() -> Self {
        Patcher {}
    }

    pub fn run_update<F>(&self, votv_exe_path: &str, mut output_callback: F) -> Result<i32, String>
    where
        F: FnMut(String),
    {
        output_callback("Checking VotV.exe path...".to_string());

        let exe_path = Path::new(votv_exe_path);
        if !exe_path.exists() {
            return Err(format!("VotV.exe not found: {}", votv_exe_path));
        }

        if exe_path.file_name().and_then(|n| n.to_str()) != Some("VotV.exe") {
            return Err("Selected file must be VotV.exe".to_string());
        }

        // Find the .pak file
        let game_dir = exe_path.parent().ok_or("Invalid exe path")?;
        let pak_path = game_dir.join("VotV/Content/Paks/VotV-WindowsNoEditor.pak");

        if !pak_path.exists() {
            return Err(format!("Pak file not found: {}", pak_path.display()));
        }

        output_callback(format!("Hashing pak file: {}", pak_path.display()));
        let pak_hash = Self::sha256_file(&pak_path)?;
        output_callback(format!("SHA256: {}", pak_hash));

        // Fetch patch manifest
        output_callback("Fetching patch manifest...".to_string());
        let manifest = Self::fetch_patch_manifest()?;

        // Determine current version
        let current_version = manifest
            .file_hash_map
            .get(&pak_hash)
            .ok_or("Game version not recognized. Your game might be modded or corrupted.")?;

        output_callback(format!("Current version: {}", current_version));
        output_callback(format!("Latest version: {}", manifest.latest));

        if current_version == &manifest.latest {
            output_callback("Already at latest version!".to_string());
            return Ok(0);
        }

        // Get patch info
        let patch_info = manifest.patches.get(current_version).ok_or(format!(
            "No patch available for version {}",
            current_version
        ))?;

        output_callback(format!("Downloading patch from: {}", patch_info.url));
        output_callback(format!("Expected SHA256: {}", patch_info.sha256));

        // Download patch
        let patch_data = Self::download_file(&patch_info.url, &mut output_callback)?;

        // Verify hash
        output_callback("Verifying patch integrity...".to_string());
        let patch_hash = Self::sha256_bytes(&patch_data);
        if patch_hash.to_uppercase() != patch_info.sha256.to_uppercase() {
            return Err("Patch SHA256 mismatch! Download may be corrupted.".to_string());
        }
        output_callback("Patch verified successfully".to_string());

        // Extract patch
        let target_parent = game_dir.parent().ok_or("Invalid game directory")?;
        output_callback(format!("Extracting patch to: {}", target_parent.display()));

        Self::extract_7z(&patch_data, target_parent, &mut output_callback)?;

        // Find and run apply_patch.sh
        output_callback("Looking for apply_patch.sh...".to_string());
        let patch_script = Self::find_patch_script(target_parent)?;
        output_callback(format!("Found: {}", patch_script.display()));

        output_callback("Applying patch...".to_string());
        Self::run_patch_script(&patch_script, &mut output_callback)?;

        output_callback("Cleaning up temporary files...".to_string());
        // Clean up extracted files
        if let Some(patch_dir) = patch_script.parent() {
            let _ = fs::remove_dir_all(patch_dir);
        }

        output_callback("Update complete!".to_string());
        Ok(0)
    }

    pub fn run_install<F>(
        &self,
        install_dir: &str,
        version: &GameVersion,
        mut output_callback: F,
    ) -> Result<i32, String>
    where
        F: FnMut(String),
    {
        output_callback(format!("Installing VotV version: {}", version.name));
        output_callback(format!("Target directory: {}", install_dir));
        output_callback(format!("Index URL: {}", version.link));
        output_callback(format!("Store URL: {}", STORE_URL));

        // Ensure desync binary exists
        output_callback("Checking for desync binary...".to_string());
        let desync_bin = Self::ensure_desync(&mut output_callback)?;
        output_callback(format!("Using desync: {}", desync_bin.display()));

        // Create install directory
        fs::create_dir_all(install_dir)
            .map_err(|e| format!("Failed to create install directory: {}", e))?;

        // Download .caidx file
        output_callback("Downloading index file...".to_string());
        let caidx_data = Self::download_file(&version.link, &mut output_callback)?;

        // Save to temp file
        let temp_dir = std::env::temp_dir().join(format!("gyeet_install_{}", std::process::id()));
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;

        let caidx_path = temp_dir.join("index.caidx");
        fs::write(&caidx_path, &caidx_data)
            .map_err(|e| format!("Failed to write caidx file: {}", e))?;

        output_callback(format!("Index saved to: {}", caidx_path.display()));

        // Run desync untar
        output_callback("Running desync to extract game files...".to_string());
        output_callback("This may take a while depending on your connection...".to_string());

        Self::run_desync(&desync_bin, &caidx_path, install_dir, &mut output_callback)?;
        output_callback("desync command complete.".to_string());

        // Cleanup
        output_callback("Cleaning up temporary files...".to_string());
        let _ = fs::remove_dir_all(&temp_dir);

        output_callback("Installation complete!".to_string());
        output_callback(format!("Game installed to: {}", install_dir));

        Ok(0)
    }

    // Helper methods
    fn sha256_file(path: &Path) -> Result<String, String> {
        let mut file = fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:X}", hasher.finalize()))
    }

    fn sha256_bytes(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:X}", hasher.finalize())
    }

    fn fetch_patch_manifest() -> Result<PatchManifest, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(PATCH_MANIFEST_URL)
            .send()
            .map_err(|e| format!("Failed to fetch manifest: {}", e))?;

        let manifest: PatchManifest = response
            .json()
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        Ok(manifest)
    }

    fn download_file<F>(url: &str, output_callback: &mut F) -> Result<Vec<u8>, String>
    where
        F: FnMut(String),
    {
        use std::io::Write;

        let client = reqwest::blocking::Client::new();
        let mut response = client
            .get(url)
            .send()
            .map_err(|e| format!("Failed to download: {}", e))?;

        let total_size = response.content_length().unwrap_or(0);
        let mut buffer = Vec::new();
        let mut downloaded = 0u64;
        let mut temp_buf = [0u8; 8192];

        loop {
            let n = response
                .read(&mut temp_buf)
                .map_err(|e| format!("Failed to read response: {}", e))?;

            if n == 0 {
                break;
            }

            buffer
                .write_all(&temp_buf[..n])
                .map_err(|e| format!("Failed to write to buffer: {}", e))?;

            downloaded += n as u64;

            // Report progress every 512KB
            if downloaded % (512 * 1024) == 0 || downloaded == total_size {
                if total_size > 0 {
                    let _percent = (downloaded as f64 / total_size as f64) * 100.0;
                    output_callback(format!("PROGRESS:{}:{}", downloaded, total_size));
                }
            }
        }

        if total_size > 0 {
            output_callback(format!(
                "Downloaded: {} / {} bytes ({:.1}%)",
                downloaded,
                total_size,
                (downloaded as f64 / total_size as f64) * 100.0
            ));
        } else {
            output_callback(format!("Downloaded: {} bytes", downloaded));
        }

        Ok(buffer)
    }

    fn extract_7z<F>(data: &[u8], target_dir: &Path, output_callback: &mut F) -> Result<(), String>
    where
        F: FnMut(String),
    {
        // Write to temp file first
        let temp_file = target_dir.join(".gyeet_temp_patch.7z");
        fs::write(&temp_file, data).map_err(|e| format!("Failed to write temp file: {}", e))?;

        // Use 7z command to extract
        let output = Command::new("7z")
            .arg("x")
            .arg("-y")
            .arg(&temp_file)
            .arg(format!("-o{}", target_dir.display()))
            .output()
            .map_err(|e| {
                format!(
                    "Failed to run 7z: {}. Make sure p7zip-full is installed.",
                    e
                )
            })?;

        // Clean up temp file
        let _ = fs::remove_file(&temp_file);

        if !output.status.success() {
            return Err(format!(
                "7z extraction failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        output_callback("Extraction complete".to_string());
        Ok(())
    }

    fn find_patch_script(search_dir: &Path) -> Result<PathBuf, String> {
        // Search for apply_patch.sh in the directory and subdirectories
        for entry in
            fs::read_dir(search_dir).map_err(|e| format!("Failed to read directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Check in subdirectory
                let script_path = path.join("apply_patch.sh");
                if script_path.exists() {
                    return Ok(script_path);
                }
            } else if path.file_name().and_then(|n| n.to_str()) == Some("apply_patch.sh") {
                return Ok(path);
            }
        }

        Err("apply_patch.sh not found after extraction".to_string())
    }

    fn run_patch_script<F>(script_path: &Path, output_callback: &mut F) -> Result<(), String>
    where
        F: FnMut(String),
    {
        let script_dir = script_path.parent().ok_or("Invalid script path")?;

        // Make scripts executable
        let _ = Command::new("chmod")
            .arg("+x")
            .arg(script_path)
            .arg(script_dir.join("hpatchz"))
            .output();

        // Run the patch script
        let mut child = Command::new("bash")
            .arg(script_path)
            .current_dir(script_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run patch script: {}", e))?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    output_callback(format!("   {}", line));
                }
            }
        }

        let status = child
            .wait()
            .map_err(|e| format!("Failed to wait for patch script: {}", e))?;

        if !status.success() {
            return Err(format!(
                "Patch script failed with code: {}",
                status.code().unwrap_or(-1)
            ));
        }

        Ok(())
    }

    fn ensure_desync<F>(output_callback: &mut F) -> Result<PathBuf, String>
    where
        F: FnMut(String),
    {
        // Check if desync exists in the current directory or PATH
        let local_desync = PathBuf::from("./desync");

        if local_desync.exists() && local_desync.is_file() {
            return Ok(local_desync);
        }

        // Check in PATH
        if let Ok(output) = Command::new("which").arg("desync").output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path_str.is_empty() {
                    return Ok(PathBuf::from(path_str));
                }
            }
        }

        // Download desync
        output_callback("Downloading desync binary...".to_string());
        output_callback(format!("From: {}", DESYNC_URL));

        let client = reqwest::blocking::Client::new();
        let mut response = client
            .get(DESYNC_URL)
            .send()
            .map_err(|e| format!("Failed to download desync: {}", e))?;

        let mut tar_gz_data = Vec::new();
        response
            .read_to_end(&mut tar_gz_data)
            .map_err(|e| format!("Failed to read desync archive: {}", e))?;

        output_callback(format!("Downloaded {} bytes", tar_gz_data.len()));

        // Extract to temp directory
        let temp_dir = std::env::temp_dir().join(format!("gyeet_desync_{}", std::process::id()));
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;

        let tar_gz_path = temp_dir.join("desync.tar.gz");
        fs::write(&tar_gz_path, &tar_gz_data)
            .map_err(|e| format!("Failed to write tar.gz: {}", e))?;

        output_callback("Extracting desync...".to_string());

        // Extract using tar command
        let output = Command::new("tar")
            .arg("-xzf")
            .arg(&tar_gz_path)
            .arg("-C")
            .arg(&temp_dir)
            .output()
            .map_err(|e| format!("Failed to run tar: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "tar extraction failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Find the desync binary
        let desync_binary = Self::find_file_recursive(&temp_dir, "desync")?;

        // Move to current directory
        let target_path = PathBuf::from("./desync");
        fs::copy(&desync_binary, &target_path)
            .map_err(|e| format!("Failed to copy desync binary: {}", e))?;

        // Make executable
        let _ = Command::new("chmod").arg("+x").arg(&target_path).output();

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);

        output_callback(format!("desync installed to: {}", target_path.display()));

        Ok(target_path)
    }

    fn find_file_recursive(dir: &Path, filename: &str) -> Result<PathBuf, String> {
        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.is_file() && path.file_name().and_then(|n| n.to_str()) == Some(filename) {
                return Ok(path);
            } else if path.is_dir() {
                if let Ok(found) = Self::find_file_recursive(&path, filename) {
                    return Ok(found);
                }
            }
        }

        Err(format!(
            "File '{}' not found in {}",
            filename,
            dir.display()
        ))
    }

    fn run_desync<F>(
        desync_bin: &Path,
        caidx_path: &Path,
        install_dir: &str,
        output_callback: &mut F,
    ) -> Result<(), String>
    where
        F: FnMut(String),
    {
        output_callback(format!(
            "🔧 Running: {} untar -i -s {} {} {}",
            desync_bin.display(),
            STORE_URL,
            caidx_path.display(),
            install_dir
        ));

        eprintln!("[DEBUG] Starting desync process...");
        output_callback("Starting unpacking process...".to_string());

        let mut child = Command::new(desync_bin)
            .arg("untar")
            .arg("--verbose")
            .arg("--no-same-owner")
            .arg("-i")
            .arg("-s")
            .arg(STORE_URL)
            .arg(caidx_path)
            .arg(install_dir)
            .arg("-n")
            .arg("16")
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run desync: {}", e))?;

        eprintln!("[DEBUG] desync spawned successfully");
        
        // Monitor stderr for activity and send periodic updates
        let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
        let running = Arc::new(Mutex::new(true));
        let running_clone = Arc::clone(&running);
        
        // Thread to read stderr
        let stderr_handle = thread::spawn(move || {
            eprintln!("[DEBUG] stderr reader thread started");
            let reader = BufReader::new(stderr);
            let mut line_count = 0;
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    line_count += 1;
                    
                    // Print every line to terminal for debugging
                    eprintln!("[STDERR {}] {}", line_count, line);
                    
                    // Look for progress indicator
                    if line.contains("Unpacking") || line.contains("%") {
                        eprintln!("[DEBUG] Found potential progress line!");
                    }
                }
            }
            
            *running_clone.lock().unwrap() = false;
            eprintln!("[DEBUG] stderr reader finished, read {} lines", line_count);
        });

        // Send periodic status updates to GUI
        let mut update_count = 0;
        while *running.lock().unwrap() {
            thread::sleep(std::time::Duration::from_secs(2));
            if *running.lock().unwrap() {
                update_count += 1;
                output_callback(format!("Unpacking in progress... ({} seconds)", update_count * 2));
                eprintln!("[DEBUG] Sent update #{}", update_count);
            }
        }

        eprintln!("[DEBUG] Waiting for stderr thread...");
        let _ = stderr_handle.join();

        eprintln!("[DEBUG] Waiting for process to exit...");
        let status = child
            .wait()
            .map_err(|e| format!("Failed to wait for desync: {}", e))?;

        eprintln!("[DEBUG] Process exited with status: {:?}", status);

        if !status.success() {
            return Err(format!("desync failed with status: {}", status));
        }

        output_callback("✅ Unpacking completed successfully".to_string());
        eprintln!("[DEBUG] run_desync completed successfully");

        Ok(())
    }
}

// Fetch available game versions from the catalog
pub fn fetch_game_versions() -> Result<Vec<GameVersion>, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(INSTALL_CATALOG_URL)
        .send()
        .map_err(|e| format!("Failed to fetch version catalog: {}", e))?;

    let versions: Vec<GameVersion> = response
        .json()
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(versions)
}