use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

const SKILL_CONTENT: &str = include_str!("../SKILL.md");
const SKILL_NAME: &str = "toll-optimizer";

fn get_skill_paths(home_dir: &Path) -> Vec<PathBuf> {
    vec![
        // Antigravity global
        home_dir
            .join(".gemini/config/skills")
            .join(SKILL_NAME)
            .join("SKILL.md"),
        // Claude global
        home_dir
            .join(".claude/skills")
            .join(SKILL_NAME)
            .join("SKILL.md"),
        // Copilot global
        home_dir
            .join(".copilot/skills")
            .join(SKILL_NAME)
            .join("SKILL.md"),
        // Generic agents global
        home_dir
            .join(".agents/skills")
            .join(SKILL_NAME)
            .join("SKILL.md"),
    ]
}

pub fn install() -> Result<()> {
    // using home_dir from std (deprecated) or we can use just a simplified approach if dirs crate is not available, but let's check Cargo.toml first.
    let home_dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory (HOME not set)"))?;

    let paths = get_skill_paths(&home_dir);

    println!("Installing '{}' skill...", SKILL_NAME);
    let mut installed = false;

    for path in paths {
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!(
                    "Warning: Failed to create directory {}: {}",
                    parent.display(),
                    e
                );
                continue;
            }
            if let Err(e) = fs::write(&path, SKILL_CONTENT) {
                eprintln!("Warning: Failed to write {}: {}", path.display(), e);
                continue;
            }
            println!("  skill installed  ->  {}", path.display());
            installed = true;
        }
    }

    if installed {
        println!("Installation complete.");
    } else {
        println!("Failed to install skill in any location.");
    }

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let home_dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory (HOME not set)"))?;

    let paths = get_skill_paths(&home_dir);

    println!("Uninstalling '{}' skill...", SKILL_NAME);
    let mut uninstalled = false;

    for path in paths {
        if path.exists() {
            if let Err(e) = fs::remove_file(&path) {
                eprintln!("Warning: Failed to remove file {}: {}", path.display(), e);
                continue;
            }
            println!("  skill removed    ->  {}", path.display());
            uninstalled = true;

            // Try to remove parent directory if empty
            if let Some(parent) = path.parent() {
                let _ = fs::remove_dir(parent); // Ignore errors if not empty
            }
        }
    }

    if uninstalled {
        println!("Uninstallation complete.");
    } else {
        println!("No existing skill installations found to remove.");
    }

    Ok(())
}
