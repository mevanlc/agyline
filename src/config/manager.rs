use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const CONFIG_DIR_ENV: &str = "AGYLINE_CONFIG_DIR";
pub const LEGACY_CONFIG_DIR_ENV: &str = "XLINE_CONFIG_DIR";

static CONFIG_DIR_OVERRIDE: OnceLock<PathBuf> = OnceLock::new();

/// Set the config directory selected by the command line.
///
/// This must be called before any config paths are resolved. The override can
/// only be set once per process.
pub fn set_config_dir_override(path: PathBuf) -> Result<(), PathBuf> {
    CONFIG_DIR_OVERRIDE.set(path)
}

/// Get the agyline config directory.
///
/// Precedence is: command-line override, `AGYLINE_CONFIG_DIR`, `XLINE_CONFIG_DIR`,
/// then `~/.config/agyline`.
pub fn config_dir() -> PathBuf {
    if let Some(path) = config_dir_override(
        CONFIG_DIR_OVERRIDE.get().map(PathBuf::as_path),
        std::env::var_os(CONFIG_DIR_ENV)
            .or_else(|| std::env::var_os(LEGACY_CONFIG_DIR_ENV))
            .as_deref(),
    ) {
        return path;
    }

    let home = dirs::home_dir().expect("could not determine home directory");
    default_config_dir(&home)
}

fn config_dir_override(cli: Option<&Path>, env: Option<&OsStr>) -> Option<PathBuf> {
    if let Some(path) = cli {
        return Some(path.to_path_buf());
    }

    env.filter(|path| !path.is_empty()).map(PathBuf::from)
}

fn default_config_dir(home: &Path) -> PathBuf {
    home.join(".config").join("agyline")
}

/// Get the path to the Antigravity settings.json file.
pub fn agy_settings_path() -> PathBuf {
    let home = dirs::home_dir().expect("could not determine home directory");
    agy_settings_path_for_home(&home)
}

/// Get the path to settings.json relative to a given home directory.
pub fn agy_settings_path_for_home(home: &Path) -> PathBuf {
    home.join(".gemini")
        .join("antigravity-cli")
        .join("settings.json")
}

#[derive(Debug, PartialEq, Eq)]
pub enum AgySetupResult {
    Configured {
        path: PathBuf,
        already_set: bool,
    },
    Conflict {
        path: PathBuf,
        existing_command: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AgyUnsetupResult {
    Removed {
        path: PathBuf,
    },
    NotConfigured {
        path: PathBuf,
    },
    DifferentCommand {
        path: PathBuf,
        existing_command: String,
    },
}

/// Configure Antigravity CLI settings.json to use `agyline` as the statusline provider.
pub fn setup_agy_statusline(settings_path: &Path, force: bool) -> Result<AgySetupResult, String> {
    let mut root: serde_json::Value = if settings_path.exists() {
        let content = fs::read_to_string(settings_path)
            .map_err(|e| format!("failed to read {}: {}", settings_path.display(), e))?;
        if content.trim().is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(&content)
                .map_err(|e| format!("failed to parse {}: {}", settings_path.display(), e))?
        }
    } else {
        serde_json::json!({})
    };

    let obj = root
        .as_object_mut()
        .ok_or_else(|| format!("root JSON in {} must be an object", settings_path.display()))?;

    if let Some(existing_sl) = obj.get("statusLine") {
        if let Some(sl_obj) = existing_sl.as_object() {
            let cmd = sl_obj.get("command").and_then(|c| c.as_str()).unwrap_or("");
            if cmd == "agyline" {
                let sl_type = sl_obj.get("type").and_then(|t| t.as_str()).unwrap_or("");
                if sl_type == "command" {
                    return Ok(AgySetupResult::Configured {
                        path: settings_path.to_path_buf(),
                        already_set: true,
                    });
                }
            } else if !cmd.is_empty() && !force {
                return Ok(AgySetupResult::Conflict {
                    path: settings_path.to_path_buf(),
                    existing_command: cmd.to_string(),
                });
            }
        } else if !force {
            return Ok(AgySetupResult::Conflict {
                path: settings_path.to_path_buf(),
                existing_command: existing_sl.to_string(),
            });
        }
    }

    // Insert or update statusLine
    let status_line = serde_json::json!({
        "type": "command",
        "command": "agyline"
    });
    obj.insert("statusLine".to_string(), status_line);

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create directory {}: {}", parent.display(), e))?;
    }

    let serialized = serde_json::to_string_pretty(&root)
        .map_err(|e| format!("failed to serialize JSON: {}", e))?;
    fs::write(settings_path, format!("{}\n", serialized))
        .map_err(|e| format!("failed to write {}: {}", settings_path.display(), e))?;

    Ok(AgySetupResult::Configured {
        path: settings_path.to_path_buf(),
        already_set: false,
    })
}

/// Remove `agyline` statusline configuration from Antigravity CLI settings.json.
pub fn unsetup_agy_statusline(settings_path: &Path) -> Result<AgyUnsetupResult, String> {
    if !settings_path.exists() {
        return Ok(AgyUnsetupResult::NotConfigured {
            path: settings_path.to_path_buf(),
        });
    }

    let content = fs::read_to_string(settings_path)
        .map_err(|e| format!("failed to read {}: {}", settings_path.display(), e))?;
    if content.trim().is_empty() {
        return Ok(AgyUnsetupResult::NotConfigured {
            path: settings_path.to_path_buf(),
        });
    }

    let mut root: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse {}: {}", settings_path.display(), e))?;

    let obj = root
        .as_object_mut()
        .ok_or_else(|| format!("root JSON in {} must be an object", settings_path.display()))?;

    let Some(existing_sl) = obj.get("statusLine") else {
        return Ok(AgyUnsetupResult::NotConfigured {
            path: settings_path.to_path_buf(),
        });
    };

    if let Some(sl_obj) = existing_sl.as_object() {
        let cmd = sl_obj.get("command").and_then(|c| c.as_str()).unwrap_or("");
        if cmd != "agyline" {
            return Ok(AgyUnsetupResult::DifferentCommand {
                path: settings_path.to_path_buf(),
                existing_command: cmd.to_string(),
            });
        }
    }

    obj.remove("statusLine");

    let serialized = serde_json::to_string_pretty(&root)
        .map_err(|e| format!("failed to serialize JSON: {}", e))?;
    fs::write(settings_path, format!("{}\n", serialized))
        .map_err(|e| format!("failed to write {}: {}", settings_path.display(), e))?;

    Ok(AgyUnsetupResult::Removed {
        path: settings_path.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    fn setup_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn config_dir_cli_override_takes_precedence() {
        let resolved = config_dir_override(
            Some(Path::new("/cli/config")),
            Some(OsStr::new("/env/config")),
        );

        assert_eq!(resolved, Some(PathBuf::from("/cli/config")));
    }

    #[test]
    fn config_dir_uses_environment_override() {
        let resolved = config_dir_override(None, Some(OsStr::new("/env/config")));

        assert_eq!(resolved, Some(PathBuf::from("/env/config")));
    }

    #[test]
    fn config_dir_defaults_beneath_home() {
        let resolved = default_config_dir(Path::new("/home/tester"));

        assert_eq!(resolved, Path::new("/home/tester/.config/agyline"));
    }

    #[test]
    fn empty_environment_override_uses_default() {
        let resolved = config_dir_override(None, Some(OsStr::new("")));

        assert_eq!(resolved, None);
    }

    #[test]
    fn test_agy_setup_creates_new_settings_file() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");

        let res = setup_agy_statusline(&settings_path, false).unwrap();
        assert_eq!(
            res,
            AgySetupResult::Configured {
                path: settings_path.clone(),
                already_set: false,
            }
        );

        let content = fs::read_to_string(&settings_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["statusLine"]["type"], "command");
        assert_eq!(json["statusLine"]["command"], "agyline");
    }

    #[test]
    fn test_agy_setup_preserves_existing_unrelated_settings() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{"otherKey": true, "terminalWidth": 120}"#,
        )
        .unwrap();

        let res = setup_agy_statusline(&settings_path, false).unwrap();
        assert_eq!(
            res,
            AgySetupResult::Configured {
                path: settings_path.clone(),
                already_set: false,
            }
        );

        let content = fs::read_to_string(&settings_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["otherKey"], true);
        assert_eq!(json["terminalWidth"], 120);
        assert_eq!(json["statusLine"]["command"], "agyline");
    }

    #[test]
    fn test_agy_setup_conflict_without_force() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{"statusLine": {"type": "command", "command": "custom-status.sh"}}"#,
        )
        .unwrap();

        let res = setup_agy_statusline(&settings_path, false).unwrap();
        assert_eq!(
            res,
            AgySetupResult::Conflict {
                path: settings_path.clone(),
                existing_command: "custom-status.sh".into(),
            }
        );

        // Verify content unchanged
        let content = fs::read_to_string(&settings_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["statusLine"]["command"], "custom-status.sh");
    }

    #[test]
    fn test_agy_setup_force_overwrites_conflict() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{"statusLine": {"type": "command", "command": "custom-status.sh"}}"#,
        )
        .unwrap();

        let res = setup_agy_statusline(&settings_path, true).unwrap();
        assert_eq!(
            res,
            AgySetupResult::Configured {
                path: settings_path.clone(),
                already_set: false,
            }
        );

        let content = fs::read_to_string(&settings_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["statusLine"]["command"], "agyline");
    }

    #[test]
    fn test_agy_setup_already_configured_is_idempotent() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{"statusLine": {"type": "command", "command": "agyline"}}"#,
        )
        .unwrap();

        let res = setup_agy_statusline(&settings_path, false).unwrap();
        assert_eq!(
            res,
            AgySetupResult::Configured {
                path: settings_path.clone(),
                already_set: true,
            }
        );
    }

    #[test]
    fn test_agy_unsetup_removes_agyline() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{"otherKey": 42, "statusLine": {"type": "command", "command": "agyline"}}"#,
        )
        .unwrap();

        let res = unsetup_agy_statusline(&settings_path).unwrap();
        assert_eq!(
            res,
            AgyUnsetupResult::Removed {
                path: settings_path.clone()
            }
        );

        let content = fs::read_to_string(&settings_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["otherKey"], 42);
        assert!(json.get("statusLine").is_none());
    }

    #[test]
    fn test_agy_unsetup_leaves_different_command_intact() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");
        fs::write(
            &settings_path,
            r#"{"statusLine": {"type": "command", "command": "other-line"}}"#,
        )
        .unwrap();

        let res = unsetup_agy_statusline(&settings_path).unwrap();
        assert_eq!(
            res,
            AgyUnsetupResult::DifferentCommand {
                path: settings_path.clone(),
                existing_command: "other-line".into(),
            }
        );

        let content = fs::read_to_string(&settings_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["statusLine"]["command"], "other-line");
    }

    #[test]
    fn test_agy_unsetup_when_not_configured() {
        let dir = setup_temp_dir();
        let settings_path = dir.path().join("settings.json");

        // Non-existent file
        let res = unsetup_agy_statusline(&settings_path).unwrap();
        assert_eq!(
            res,
            AgyUnsetupResult::NotConfigured {
                path: settings_path.clone()
            }
        );

        // File without statusLine
        fs::write(&settings_path, r#"{"someConfig": "value"}"#).unwrap();
        let res2 = unsetup_agy_statusline(&settings_path).unwrap();
        assert_eq!(
            res2,
            AgyUnsetupResult::NotConfigured {
                path: settings_path.clone()
            }
        );
    }
}
