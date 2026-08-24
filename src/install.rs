//! Installation and uninstallation logic for the `toll-optimizer` LLM skill.
//!
//! This module provides functions to install the skill definition into configuration
//! directories of supported AI assistants/agents only when those directories already exist.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

const SKILL_CONTENT: &str = include_str!("../SKILL.md");
const SKILL_NAME: &str = "toll-optimizer";

struct SkillTarget {
    llm_dir: &'static str,
    skill_rel_path: &'static str,
}

const SKILL_TARGETS: &[SkillTarget] = &[
    SkillTarget {
        llm_dir: ".gemini",
        skill_rel_path: ".gemini/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".gemini/config",
        skill_rel_path: ".gemini/config/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".agents",
        skill_rel_path: ".agents/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".config/agents",
        skill_rel_path: ".config/agents/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".config/opencode",
        skill_rel_path: ".config/opencode/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".hermes",
        skill_rel_path: ".hermes/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".config/devin",
        skill_rel_path: ".config/devin/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".claude",
        skill_rel_path: ".claude/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".aider",
        skill_rel_path: ".aider/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".copilot",
        skill_rel_path: ".copilot/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".openclaw",
        skill_rel_path: ".openclaw/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".factory",
        skill_rel_path: ".factory/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".trae",
        skill_rel_path: ".trae/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".trae-cn",
        skill_rel_path: ".trae-cn/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".kiro",
        skill_rel_path: ".kiro/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".pi",
        skill_rel_path: ".pi/agent/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".codebuddy",
        skill_rel_path: ".codebuddy/skills/toll-optimizer/SKILL.md",
    },
    SkillTarget {
        llm_dir: ".kimi",
        skill_rel_path: ".kimi/skills/toll-optimizer/SKILL.md",
    },
];

/// Installs the skill file into all existing LLM directories found relative to `base_dir`.
///
/// Returns the number of locations where the skill was successfully installed.
pub fn install_skills(base_dir: &Path) -> usize {
    let mut installed_count = 0;

    for target in SKILL_TARGETS {
        let llm_dir_path = base_dir.join(target.llm_dir);
        if !llm_dir_path.is_dir() {
            continue;
        }

        let target_file = base_dir.join(target.skill_rel_path);

        if let Some(parent) = target_file.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            eprintln!(
                "Warning: Failed to create directory {}: {}",
                parent.display(),
                e
            );
            continue;
        }

        match fs::write(&target_file, SKILL_CONTENT) {
            Ok(_) => {
                println!("  skill installed  ->  {}", target_file.display());
                installed_count += 1;
            }
            Err(e) => {
                eprintln!("Warning: Failed to write {}: {}", target_file.display(), e);
            }
        }
    }

    installed_count
}

/// Uninstalls the skill file from all target locations relative to `base_dir`.
///
/// Returns the number of skill files that were successfully removed.
pub fn uninstall_skills(base_dir: &Path) -> usize {
    let mut uninstalled_count = 0;

    for target in SKILL_TARGETS {
        let target_file = base_dir.join(target.skill_rel_path);
        if target_file.exists() {
            if let Err(e) = fs::remove_file(&target_file) {
                eprintln!(
                    "Warning: Failed to remove file {}: {}",
                    target_file.display(),
                    e
                );
                continue;
            }
            println!("  skill removed    ->  {}", target_file.display());
            uninstalled_count += 1;

            // Try to remove parent directory if empty
            if let Some(parent) = target_file.parent() {
                let _ = fs::remove_dir(parent); // Ignore errors if not empty
            }
        }
    }

    uninstalled_count
}

/// Installs the `toll-optimizer` skill into existing LLM directories in the user's home directory.
pub fn install() -> Result<()> {
    let home_dir = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .ok_or_else(|| {
            anyhow::anyhow!("Could not determine home directory (HOME or USERPROFILE not set)")
        })?;

    println!("Installing '{}' skill...", SKILL_NAME);
    let count = install_skills(&home_dir);

    if count > 0 {
        println!(
            "✓ Successfully installed {} skill to {} location(s).",
            SKILL_NAME, count
        );
    } else {
        println!("No existing LLM directories found. No skill files were installed.");
    }

    Ok(())
}

/// Uninstalls the `toll-optimizer` skill from all known locations in the user's home directory.
pub fn uninstall() -> Result<()> {
    let home_dir = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .ok_or_else(|| {
            anyhow::anyhow!("Could not determine home directory (HOME or USERPROFILE not set)")
        })?;

    println!("Uninstalling '{}' skill...", SKILL_NAME);
    let count = uninstall_skills(&home_dir);

    if count > 0 {
        println!(
            "✓ Successfully uninstalled {} skill from {} location(s).",
            SKILL_NAME, count
        );
    } else {
        println!("No existing skill installations found to remove.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_dir(test_name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        dir.push(format!(
            "toll_optimizer_install_test_{}_{}",
            test_name, unique_id
        ));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn test_install_skills_empty_directory() {
        let test_dir = get_test_dir("empty");
        let count = install_skills(&test_dir);
        assert_eq!(count, 0);

        // Verify no LLM directories were created
        assert!(!test_dir.join(".claude").exists());
        assert!(!test_dir.join(".gemini").exists());
        assert!(!test_dir.join(".agents").exists());
        assert!(!test_dir.join(".aider").exists());

        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_install_skills_only_existing_directories() {
        let test_dir = get_test_dir("existing");
        fs::create_dir_all(test_dir.join(".claude")).expect("failed to create .claude");
        fs::create_dir_all(test_dir.join(".config").join("devin"))
            .expect("failed to create .config/devin");

        let count = install_skills(&test_dir);
        assert_eq!(count, 2);

        let claude_skill = test_dir
            .join(".claude")
            .join("skills")
            .join(SKILL_NAME)
            .join("SKILL.md");
        let devin_skill = test_dir
            .join(".config")
            .join("devin")
            .join("skills")
            .join(SKILL_NAME)
            .join("SKILL.md");
        assert!(claude_skill.exists());
        assert!(devin_skill.exists());

        // Verify other LLM directories were not created
        assert!(!test_dir.join(".gemini").exists());
        assert!(!test_dir.join(".agents").exists());
        assert!(!test_dir.join(".aider").exists());
        assert!(!test_dir.join(".copilot").exists());

        let uninstalled = uninstall_skills(&test_dir);
        assert_eq!(uninstalled, 2);
        assert!(!claude_skill.exists());
        assert!(!devin_skill.exists());

        let _ = fs::remove_dir_all(test_dir);
    }
}
