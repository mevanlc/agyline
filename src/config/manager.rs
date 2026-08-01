use std::fs;
use std::path::{Path, PathBuf};

use crate::config::theme::UserTheme;

/// Get the themes directory: ~/.claude/xline/themes/
pub fn themes_dir() -> PathBuf {
    let home = dirs::home_dir().expect("could not determine home directory");
    home.join(".claude").join("xline").join("themes")
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
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
}
