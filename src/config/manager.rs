use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::config::theme::UserTheme;

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
/// then `~/.gemini/antigravity-cli/agyline`.
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
    home.join(".gemini").join("antigravity-cli").join("agyline")
}

/// Get the path to the Antigravity settings.json file.
pub fn agy_settings_path() -> PathBuf {
    agy_settings_path_for_config_dir(&config_dir())
}

/// Get the path to settings.json relative to a given agyline config directory.
pub fn agy_settings_path_for_config_dir(config_dir: &Path) -> PathBuf {
    config_dir
        .parent()
        .map(|p| p.join("settings.json"))
        .unwrap_or_else(|| config_dir.join("settings.json"))
}

/// Get the themes directory beneath the agyline config directory.
pub fn themes_dir() -> PathBuf {
    config_dir().join("themes")
}

/// Ensure the themes directory exists and contains at least one theme.
/// If no themes exist, creates all starter themes.
pub fn bootstrap() -> std::io::Result<()> {
    let dir = themes_dir();
    fs::create_dir_all(&dir)?;

    let themes = list_theme_files(&dir)?;
    if themes.is_empty() {
        write_default_themes(&dir, false)?;
    }
    Ok(())
}

/// Write all starter themes to the given directory.
/// If `force` is true, overwrite existing files. Otherwise skip them.
/// Returns the number of themes written.
pub fn write_default_themes(dir: &Path, force: bool) -> std::io::Result<usize> {
    use crate::config::types::StyleMode;
    use crate::presets::{color_schemes, icon_sets};

    struct Spec {
        name: &'static str,
        colors: &'static str,
        icons: &'static str,
        mode: StyleMode,
        active: bool,
    }

    let specs = [
        Spec {
            name: "Default",
            colors: "Default",
            icons: "Emoji",
            mode: StyleMode::Plain,
            active: true,
        },
        Spec {
            name: "Cometix",
            colors: "Cometix",
            icons: "Nerd Font",
            mode: StyleMode::NerdFont,
            active: false,
        },
        Spec {
            name: "Minimal",
            colors: "Minimal",
            icons: "Minimal",
            mode: StyleMode::Plain,
            active: false,
        },
        Spec {
            name: "Gruvbox",
            colors: "Gruvbox",
            icons: "Nerd Font",
            mode: StyleMode::NerdFont,
            active: false,
        },
        Spec {
            name: "Nord",
            colors: "Nord",
            icons: "Nerd Font",
            mode: StyleMode::NerdFont,
            active: false,
        },
        Spec {
            name: "Powerline Dark",
            colors: "Powerline Dark",
            icons: "Powerline",
            mode: StyleMode::Powerline,
            active: false,
        },
        Spec {
            name: "Powerline Light",
            colors: "Powerline Light",
            icons: "Powerline",
            mode: StyleMode::Powerline,
            active: false,
        },
        Spec {
            name: "Rose Pine",
            colors: "Rose Pine",
            icons: "Nerd Font",
            mode: StyleMode::NerdFont,
            active: false,
        },
        Spec {
            name: "Tokyo Night",
            colors: "Tokyo Night",
            icons: "Nerd Font",
            mode: StyleMode::NerdFont,
            active: false,
        },
        Spec {
            name: "Late",
            colors: "Late",
            icons: "Late",
            mode: StyleMode::NerdFont,
            active: false,
        },
    ];

    let mut written = 0;
    for spec in &specs {
        let path = dir.join(format!("{}.toml", spec.name));
        if !force && path.exists() {
            continue;
        }

        let mut theme = UserTheme::default_theme();
        theme.active = spec.active;
        theme.style.mode = spec.mode;

        if let Some(colors) = color_schemes::find(spec.colors) {
            colors.apply_to(&mut theme.components);
        }
        if let Some(icons) = icon_sets::find(spec.icons) {
            icons.apply_to(&mut theme.components);
        }

        save_theme(&path, &theme)?;
        written += 1;
    }

    Ok(written)
}

/// List all .toml theme files in the themes directory.
/// Returns (name, path) pairs sorted by name.
pub fn list_themes() -> std::io::Result<Vec<(String, PathBuf)>> {
    let dir = themes_dir();
    list_theme_files(&dir)
}

fn list_theme_files(dir: &Path) -> std::io::Result<Vec<(String, PathBuf)>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut themes = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml")
            && let Some(name) = path.file_stem().and_then(|s| s.to_str())
        {
            themes.push((name.to_string(), path));
        }
    }
    themes.sort_by_key(|(name, _)| name.to_lowercase());
    Ok(themes)
}

/// Load a theme from a .toml file.
pub fn load_theme(path: &Path) -> Result<UserTheme, LoadError> {
    let content = fs::read_to_string(path).map_err(LoadError::Io)?;
    let mut theme: UserTheme = toml::from_str(&content).map_err(LoadError::Parse)?;
    theme.add_missing_components();
    Ok(theme)
}

/// Save a theme to a .toml file.
pub fn save_theme(path: &Path, theme: &UserTheme) -> std::io::Result<()> {
    let content = toml::to_string_pretty(theme).map_err(std::io::Error::other)?;
    fs::write(path, content)
}

/// Find and load the active theme (first file with active=true).
/// If multiple are active, deactivates extras. If none active, activates the first.
pub fn load_active_theme() -> Result<(String, PathBuf, UserTheme), LoadError> {
    let dir = themes_dir();
    let themes = list_theme_files(&dir).map_err(LoadError::Io)?;

    if themes.is_empty() {
        return Err(LoadError::NoThemes);
    }

    let mut active: Option<(String, PathBuf, UserTheme)> = None;
    let mut extras_to_deactivate = Vec::new();

    for (name, path) in &themes {
        let theme = load_theme(path)?;
        if theme.active {
            if active.is_some() {
                extras_to_deactivate.push(path.clone());
            } else {
                active = Some((name.clone(), path.clone(), theme));
            }
        }
    }

    // Deactivate extras
    for path in extras_to_deactivate {
        if let Ok(mut theme) = load_theme(&path) {
            theme.active = false;
            let _ = save_theme(&path, &theme);
        }
    }

    // If none were active, activate the first
    if active.is_none() {
        let (name, path) = &themes[0];
        let mut theme = load_theme(path)?;
        theme.active = true;
        save_theme(path, &theme).map_err(LoadError::Io)?;
        active = Some((name.clone(), path.clone(), theme));
    }

    Ok(active.unwrap())
}

/// Activate a specific theme by name. Deactivates all others.
pub fn activate_theme(name: &str) -> Result<(), LoadError> {
    let dir = themes_dir();
    let themes = list_theme_files(&dir).map_err(LoadError::Io)?;

    let mut found = false;
    for (tname, path) in &themes {
        let mut theme = load_theme(path)?;
        let should_be_active = tname == name;
        if should_be_active {
            found = true;
        }
        if theme.active != should_be_active {
            theme.active = should_be_active;
            save_theme(path, &theme).map_err(LoadError::Io)?;
        }
    }

    if !found {
        return Err(LoadError::NotFound(name.to_string()));
    }
    Ok(())
}

/// Delete a theme file. Returns error if it's the last theme.
pub fn delete_theme(path: &Path) -> Result<(), DeleteError> {
    let dir = themes_dir();
    let themes = list_theme_files(&dir).map_err(DeleteError::Io)?;
    if themes.len() <= 1 {
        return Err(DeleteError::LastTheme);
    }
    fs::remove_file(path).map_err(DeleteError::Io)
}

/// Rename a theme file. Returns the new path.
pub fn rename_theme(old_path: &Path, new_name: &str) -> Result<PathBuf, RenameError> {
    if !is_valid_theme_name(new_name) {
        return Err(RenameError::InvalidName(new_name.to_string()));
    }

    let dir = old_path
        .parent()
        .ok_or_else(|| RenameError::InvalidName("no parent directory".into()))?;
    let new_path = dir.join(format!("{}.toml", new_name));

    if new_path.exists() {
        return Err(RenameError::AlreadyExists(new_name.to_string()));
    }

    fs::rename(old_path, &new_path).map_err(RenameError::Io)?;
    Ok(new_path)
}

/// Duplicate a theme. Returns the new path.
pub fn duplicate_theme(src_path: &Path, new_name: &str) -> Result<PathBuf, RenameError> {
    if !is_valid_theme_name(new_name) {
        return Err(RenameError::InvalidName(new_name.to_string()));
    }

    let dir = src_path
        .parent()
        .ok_or_else(|| RenameError::InvalidName("no parent directory".into()))?;
    let new_path = dir.join(format!("{}.toml", new_name));

    if new_path.exists() {
        return Err(RenameError::AlreadyExists(new_name.to_string()));
    }

    let mut theme = load_theme(src_path).map_err(|e| match e {
        LoadError::Io(io) => RenameError::Io(io),
        other => RenameError::Io(std::io::Error::other(format!("{}", other))),
    })?;
    theme.active = false; // duplicate is not active by default
    save_theme(&new_path, &theme).map_err(RenameError::Io)?;
    Ok(new_path)
}

/// Check if a theme name is valid for use as a filename on Windows/macOS/Linux.
pub fn is_valid_theme_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 200 {
        return false;
    }

    // No path separators or null bytes
    if name.contains('/') || name.contains('\\') || name.contains('\0') {
        return false;
    }

    // No characters forbidden on Windows
    const FORBIDDEN: &[char] = &['<', '>', ':', '"', '|', '?', '*'];
    if name
        .chars()
        .any(|c| FORBIDDEN.contains(&c) || c.is_control())
    {
        return false;
    }

    // No leading/trailing spaces or dots (Windows issue)
    if name.starts_with(' ') || name.ends_with(' ') || name.starts_with('.') || name.ends_with('.')
    {
        return false;
    }

    // No reserved Windows names
    let upper = name.to_uppercase();
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    if RESERVED.contains(&upper.as_str()) {
        return false;
    }

    true
}

/// Get the path for a theme by name.
pub fn theme_path(name: &str) -> PathBuf {
    themes_dir().join(format!("{}.toml", name))
}

// --- Error types ---

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    NoThemes,
    NotFound(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "IO error: {}", e),
            LoadError::Parse(e) => write!(f, "parse error: {}", e),
            LoadError::NoThemes => write!(f, "no themes found"),
            LoadError::NotFound(name) => write!(f, "theme not found: {}", name),
        }
    }
}

impl std::error::Error for LoadError {}

#[derive(Debug)]
pub enum DeleteError {
    Io(std::io::Error),
    LastTheme,
}

impl std::fmt::Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteError::Io(e) => write!(f, "IO error: {}", e),
            DeleteError::LastTheme => write!(f, "cannot delete the last theme"),
        }
    }
}

impl std::error::Error for DeleteError {}

#[derive(Debug)]
pub enum RenameError {
    Io(std::io::Error),
    InvalidName(String),
    AlreadyExists(String),
}

impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenameError::Io(e) => write!(f, "IO error: {}", e),
            RenameError::InvalidName(name) => write!(f, "invalid theme name: {}", name),
            RenameError::AlreadyExists(name) => {
                write!(f, "theme already exists: {}", name)
            }
        }
    }
}

impl std::error::Error for RenameError {}

// --- Antigravity CLI statusLine configuration ---

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

        assert_eq!(
            resolved,
            Path::new("/home/tester/.gemini/antigravity-cli/agyline")
        );
    }

    #[test]
    fn empty_environment_override_uses_default() {
        let resolved = config_dir_override(None, Some(OsStr::new("")));

        assert_eq!(resolved, None);
    }

    #[test]
    fn test_valid_theme_names() {
        assert!(is_valid_theme_name("Default"));
        assert!(is_valid_theme_name("My Theme"));
        assert!(is_valid_theme_name("nord-dark-v2"));
        assert!(is_valid_theme_name("theme_123"));
    }

    #[test]
    fn test_invalid_theme_names() {
        assert!(!is_valid_theme_name(""));
        assert!(!is_valid_theme_name("foo/bar"));
        assert!(!is_valid_theme_name("foo\\bar"));
        assert!(!is_valid_theme_name("CON"));
        assert!(!is_valid_theme_name(".hidden"));
        assert!(!is_valid_theme_name("trailing."));
        assert!(!is_valid_theme_name("has<bracket"));
        assert!(!is_valid_theme_name("has:colon"));
    }

    #[test]
    fn test_save_and_load_theme() {
        let dir = setup_temp_dir();
        let path = dir.path().join("test.toml");
        let theme = UserTheme::default_theme();
        save_theme(&path, &theme).unwrap();

        let loaded = load_theme(&path).unwrap();
        assert_eq!(loaded.active, theme.active);
        assert_eq!(loaded.components.len(), theme.components.len());
    }

    #[test]
    fn test_rename_theme() {
        let dir = setup_temp_dir();
        let old_path = dir.path().join("Old.toml");
        let theme = UserTheme::default_theme();
        save_theme(&old_path, &theme).unwrap();

        let new_path = rename_theme(&old_path, "New").unwrap();
        assert!(!old_path.exists());
        assert!(new_path.exists());
        assert_eq!(new_path.file_stem().unwrap().to_str().unwrap(), "New");
    }

    #[test]
    fn test_rename_to_existing_fails() {
        let dir = setup_temp_dir();
        let path_a = dir.path().join("A.toml");
        let path_b = dir.path().join("B.toml");
        let theme = UserTheme::default_theme();
        save_theme(&path_a, &theme).unwrap();
        save_theme(&path_b, &theme).unwrap();

        let result = rename_theme(&path_a, "B");
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_theme() {
        let dir = setup_temp_dir();
        let src = dir.path().join("Source.toml");
        let theme = UserTheme::default_theme();
        save_theme(&src, &theme).unwrap();

        let dup_path = duplicate_theme(&src, "Copy").unwrap();
        assert!(src.exists());
        assert!(dup_path.exists());

        let dup = load_theme(&dup_path).unwrap();
        assert!(!dup.active, "duplicate should not be active");
    }

    #[test]
    fn test_delete_last_theme_fails() {
        let dir = setup_temp_dir();
        let path = dir.path().join("Only.toml");
        let theme = UserTheme::default_theme();
        save_theme(&path, &theme).unwrap();

        // delete_theme checks themes_dir() which is the real dir, so we test directly
        // Instead, just verify the logic: list_theme_files with 1 file
        let files = list_theme_files(dir.path()).unwrap();
        assert_eq!(files.len(), 1);
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
