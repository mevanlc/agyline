use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::catalog::{Catalog, Result};

/// The exact bytes read are the optimistic-concurrency token, including comments.
pub struct Store {
    pub path: PathBuf,
    pub base: Catalog,
    original: Option<Vec<u8>>,
    uncertain: bool,
    #[cfg(test)]
    fault: Option<WriteFault>,
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum WriteFault {
    BeforeReplace,
    AfterReplace,
}

impl Store {
    pub fn open(root: &Path) -> Result<Self> {
        let path = root.join("config.toml");
        let original = read_optional(&path)?;
        let base = match &original {
            Some(bytes) => {
                let text =
                    std::str::from_utf8(bytes).map_err(|e| format!("{}: {e}", path.display()))?;
                let c: Catalog =
                    toml::from_str(text).map_err(|e| format!("{}: {e}", path.display()))?;
                c.validate()
                    .map_err(|e| format!("{}:\n{e}", path.display()))?;
                c
            }
            None => Catalog::default(),
        };
        Ok(Self {
            path,
            base,
            original,
            uncertain: false,
            #[cfg(test)]
            fault: None,
        })
    }

    pub fn save(&mut self, catalog: &Catalog) -> Result<()> {
        if self.uncertain {
            return Err(
                "The previous write may have succeeded. Reload the catalog before saving again."
                    .into(),
            );
        }
        catalog.validate()?;
        let serialized = toml::to_string_pretty(catalog).map_err(|e| e.to_string())?;
        let parent = self.path.parent().ok_or("Config path has no parent")?;
        fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        let lock_path = parent.join("config.toml.lock");
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|e| format!("{}: {e}", lock_path.display()))?;
        lock.try_lock()
            .map_err(|e| format!("Catalog is being saved by another process: {e}"))?;
        if read_optional(&self.path)? != self.original {
            return Err("Catalog changed outside this editor. Drafts are retained; reload or export drafts before saving.".into());
        }
        let mut temporary = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        if let Ok(metadata) = fs::metadata(&self.path) {
            temporary
                .as_file()
                .set_permissions(metadata.permissions())
                .map_err(|e| e.to_string())?;
        }
        temporary
            .write_all(serialized.as_bytes())
            .map_err(|e| e.to_string())?;
        temporary.as_file().sync_all().map_err(|e| e.to_string())?;
        #[cfg(test)]
        if self.fault == Some(WriteFault::BeforeReplace) {
            return Err("Injected failure before replacement".into());
        }
        temporary
            .persist(&self.path)
            .map_err(|e| format!("Could not replace {}: {e}", self.path.display()))?;
        #[cfg(test)]
        if self.fault == Some(WriteFault::AfterReplace) {
            self.uncertain = true;
            return Err("Catalog replaced; injected durability failure. Reload required.".into());
        }
        // Once replacement happened, a durability failure must not be retried blindly.
        #[cfg(unix)]
        if let Err(e) = File::open(parent).and_then(|f| f.sync_all()) {
            self.uncertain = true;
            return Err(format!(
                "Catalog replaced, but directory sync failed: {e}. Reload required."
            ));
        }
        self.original = Some(serialized.into_bytes());
        self.base = catalog.clone();
        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        let root = self.path.parent().ok_or("Config path has no parent")?;
        *self = Self::open(root)?;
        Ok(())
    }

    pub fn export_drafts(&self, catalog: &Catalog) -> Result<PathBuf> {
        let data = toml::to_string_pretty(catalog).map_err(|e| e.to_string())?;
        let parent = self.path.parent().ok_or("Config path has no parent")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let mut file = tempfile::Builder::new()
            .prefix("drafts-")
            .suffix(".toml")
            .tempfile_in(parent)
            .map_err(|e| e.to_string())?;
        file.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
        file.as_file().sync_all().map_err(|e| e.to_string())?;
        let (_, path) = file.keep().map_err(|e| e.to_string())?;
        Ok(path)
    }
}

fn read_optional(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::catalog::{Kind, ResourceRef};

    #[test]
    fn opening_and_rendering_are_read_only_even_with_old_theme_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("themes")).unwrap();
        fs::write(dir.path().join("themes/Old.toml"), "not a theme").unwrap();
        let store = Store::open(dir.path()).unwrap();
        store.base.resolve(&store.base.active_theme).unwrap();
        assert!(!store.path.exists());
        assert!(!dir.path().join("config.toml.lock").exists());
    }

    #[test]
    fn atomic_save_roundtrips_and_detects_external_changes() {
        let dir = tempfile::tempdir().unwrap();
        let mut first = Store::open(dir.path()).unwrap();
        let mut second = Store::open(dir.path()).unwrap();
        let mut c = first.base.clone();
        let theme = c.get(Kind::Theme, &ResourceRef::builtin("Nord")).unwrap();
        c.put("Work".into(), theme);
        c.active_theme = ResourceRef::user("Work");
        first.save(&c).unwrap();
        assert_eq!(Store::open(dir.path()).unwrap().base, c);
        assert!(
            second
                .save(&Catalog::default())
                .unwrap_err()
                .contains("changed outside")
        );
        assert_eq!(Store::open(dir.path()).unwrap().base, c);
        fs::write(&first.path, "corrupted externally").unwrap();
        assert!(first.save(&c).unwrap_err().contains("changed outside"));
        assert_eq!(
            fs::read_to_string(&first.path).unwrap(),
            "corrupted externally"
        );
    }

    #[test]
    fn validation_failure_leaves_existing_catalog_intact() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path()).unwrap();
        store.save(&Catalog::default()).unwrap();
        let before = fs::read(&store.path).unwrap();
        let mut invalid = store.base.clone();
        invalid.active_theme = ResourceRef::user("missing");
        assert!(store.save(&invalid).is_err());
        assert_eq!(fs::read(&store.path).unwrap(), before);
    }

    #[test]
    fn lock_conflict_preserves_the_previous_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path()).unwrap();
        store.save(&Catalog::default()).unwrap();
        let lock = OpenOptions::new()
            .write(true)
            .open(dir.path().join("config.toml.lock"))
            .unwrap();
        lock.lock().unwrap();
        assert!(
            store
                .save(&Catalog::default())
                .unwrap_err()
                .contains("another process")
        );
    }
}

#[cfg(test)]
mod failure_tests {
    use super::*;
    use crate::config::catalog::ResourceRef;
    #[test]
    fn pre_replace_failure_preserves_bytes_post_replace_failure_requires_reload() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(dir.path()).unwrap();
        store.save(&Catalog::default()).unwrap();
        let old = fs::read(&store.path).unwrap();
        let mut changed = store.base.clone();
        changed.active_theme = ResourceRef::builtin("Nord");
        store.fault = Some(WriteFault::BeforeReplace);
        assert!(store.save(&changed).is_err());
        assert_eq!(fs::read(&store.path).unwrap(), old);
        assert_eq!(store.base.active_theme, ResourceRef::builtin("Default"));
        store.fault = Some(WriteFault::AfterReplace);
        assert!(
            store
                .save(&changed)
                .unwrap_err()
                .contains("Reload required")
        );
        assert_eq!(Store::open(dir.path()).unwrap().base, changed);
        assert_eq!(store.base.active_theme, ResourceRef::builtin("Default"));
        store.fault = None;
        assert!(store.save(&changed).unwrap_err().contains("Reload"));
        store.reload().unwrap();
        assert_eq!(store.base, changed);
        store.save(&changed).unwrap();
    }
    #[test]
    fn competing_writers_never_lose_a_committed_change() {
        let dir = tempfile::tempdir().unwrap();
        let a = Store::open(dir.path()).unwrap();
        let b = Store::open(dir.path()).unwrap();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let handles = [a, b]
            .into_iter()
            .enumerate()
            .map(|(i, mut store)| {
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    let c = Catalog {
                        active_theme: ResourceRef::builtin(if i == 0 { "Nord" } else { "Minimal" }),
                        ..Default::default()
                    };
                    barrier.wait();
                    store.save(&c).map(|_| c)
                })
            })
            .collect::<Vec<_>>();
        let outcomes = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(outcomes.iter().filter(|r| r.is_ok()).count(), 1);
        assert_eq!(
            Store::open(dir.path()).unwrap().base,
            *outcomes.iter().find_map(|r| r.as_ref().ok()).unwrap()
        );
    }
}
