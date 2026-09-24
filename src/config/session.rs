use std::collections::BTreeSet;

use super::catalog::*;
use super::store::Store;

#[derive(Debug, Clone)]
struct Rename {
    kind: Kind,
    old: ResourceRef,
    new: ResourceRef,
}

/// Draft catalog plus the indivisible operations needed for scoped commits.
pub struct Session {
    pub store: Store,
    pub draft: Catalog,
    groups: Vec<BTreeSet<(Kind, ResourceRef)>>,
    renames: Vec<Rename>,
}

impl Session {
    pub fn new(store: Store) -> Self {
        Self {
            draft: store.base.clone(),
            store,
            groups: Vec::new(),
            renames: Vec::new(),
        }
    }
    pub fn dirty(&self, kind: Kind, r: &ResourceRef) -> bool {
        if r.source == Source::Builtin {
            return false;
        }
        self.draft.get(kind, r).ok() != self.store.base.get(kind, r).ok()
            || self
                .renames
                .iter()
                .any(|op| op.kind == kind && op.new == *r)
    }
    pub fn any_dirty(&self) -> bool {
        self.draft != self.store.base
    }

    fn check_new_name(&self, kind: Kind, name: &str, replacing: Option<&str>) -> Result<()> {
        self.draft.check_new_name(kind, name, replacing)?;
        if self.renames.iter().any(|op| {
            op.kind == kind
                && op.old.name.to_lowercase() == name.to_lowercase()
                && Some(op.new.name.as_str()) != replacing
        }) {
            return Err("Save the pending rename before reusing its former name".into());
        }
        Ok(())
    }

    pub fn copy(&mut self, kind: Kind, r: &ResourceRef, name: &str) -> Result<ResourceRef> {
        self.check_new_name(kind, name, None)?;
        let value = self.draft.get(kind, r)?;
        self.draft.put(name.into(), value);
        Ok(ResourceRef::user(name))
    }

    pub fn duplicate_theme(
        &mut self,
        r: &ResourceRef,
        name: &str,
        full: bool,
    ) -> Result<ResourceRef> {
        self.check_new_name(Kind::Theme, name, None)?;
        let mut t = self.draft.theme(r)?;
        let target = ResourceRef::user(name);
        let mut group = BTreeSet::from([(Kind::Theme, target.clone())]);
        if full {
            for kind in [Kind::Components, Kind::Icons, Kind::Colors] {
                let suggested = self.draft.suggested_name(kind, name);
                let copied = self.copy(kind, t.reference(kind), &suggested)?;
                *t.reference_mut(kind) = copied.clone();
                group.insert((kind, copied));
            }
        }
        self.draft.put(name.into(), Resource::Theme(t));
        if full {
            self.groups.push(group);
        }
        Ok(target)
    }

    pub fn customize(
        &mut self,
        theme: &ResourceRef,
        kind: Kind,
        name: &str,
    ) -> Result<(ResourceRef, ResourceRef)> {
        if kind == Kind::Theme {
            return Err("Choose a config to customize".into());
        }
        let source = self.draft.theme(theme)?.reference(kind).clone();
        self.customize_from(theme, kind, &source, name)
    }

    pub fn customize_from(
        &mut self,
        theme: &ResourceRef,
        kind: Kind,
        source: &ResourceRef,
        name: &str,
    ) -> Result<(ResourceRef, ResourceRef)> {
        if kind == Kind::Theme {
            return Err("Choose a config to customize".into());
        }
        // Validate before adding either draft.
        self.check_new_name(kind, name, None)?;
        let mut t = self.draft.theme(theme)?;
        let theme_ref = if theme.source == Source::Builtin {
            let n = self.draft.suggested_name(Kind::Theme, &theme.name);
            ResourceRef::user(n)
        } else {
            theme.clone()
        };
        let copied = self.copy(kind, source, name)?;
        *t.reference_mut(kind) = copied.clone();
        self.draft.put(theme_ref.name.clone(), Resource::Theme(t));
        self.groups.push(BTreeSet::from([
            (Kind::Theme, theme_ref.clone()),
            (kind, copied.clone()),
        ]));
        Ok((theme_ref, copied))
    }

    pub fn associate(
        &mut self,
        theme: &ResourceRef,
        kind: Kind,
        config: ResourceRef,
    ) -> Result<ResourceRef> {
        if kind == Kind::Theme {
            return Err("Themes reference only configs".into());
        }
        self.draft.get(kind, &config)?;
        let target = if theme.source == Source::Builtin {
            let name = self.draft.suggested_name(Kind::Theme, &theme.name);
            self.duplicate_theme(theme, &name, false)?
        } else {
            theme.clone()
        };
        let t = self
            .draft
            .themes
            .get_mut(&target.name)
            .ok_or("Theme missing")?;
        *t.reference_mut(kind) = config;
        Ok(target)
    }

    pub fn rename(&mut self, kind: Kind, r: &ResourceRef, name: &str) -> Result<ResourceRef> {
        if r.source == Source::Builtin {
            return Err("Built-ins are read-only; duplicate or customize instead".into());
        }
        self.check_new_name(kind, name, Some(&r.name))?;
        if name == r.name {
            return Ok(r.clone());
        }
        let value = self.draft.get(kind, r)?;
        let new = ResourceRef::user(name);
        self.draft.remove(kind, &r.name);
        self.draft.put(name.into(), value);
        self.draft.rename_refs(kind, r, &new);
        for group in &mut self.groups {
            if group.remove(&(kind, r.clone())) {
                group.insert((kind, new.clone()));
            }
        }
        // Consecutive renames are one operation relative to the saved base.
        if let Some(op) = self
            .renames
            .iter_mut()
            .find(|op| op.kind == kind && op.new == *r)
        {
            op.new = new.clone();
        } else {
            self.renames.push(Rename {
                kind,
                old: r.clone(),
                new: new.clone(),
            });
        }
        Ok(new)
    }

    pub fn delete(&mut self, kind: Kind, r: &ResourceRef) -> Result<()> {
        if r.source == Source::Builtin {
            return Err("Built-ins are read-only".into());
        }
        if kind == Kind::Theme
            && (self.store.base.active_theme == *r || self.draft.active_theme == *r)
        {
            return Err("Activate a replacement theme before deleting the active theme".into());
        }
        let dependents = self.draft.dependents(kind, r);
        if !dependents.is_empty() {
            return Err(format!(
                "Used by: {}. Reassign these themes first.",
                dependents
                    .iter()
                    .map(|r| r.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        self.draft.remove(kind, &r.name);
        Ok(())
    }

    fn scope(&self, kind: Kind, r: &ResourceRef) -> Result<BTreeSet<(Kind, ResourceRef)>> {
        let mut set = BTreeSet::from([(kind, r.clone())]);
        loop {
            let old = set.len();
            for (k, reference) in set.clone() {
                if k == Kind::Theme
                    && let Ok(theme) = self.draft.theme(&reference)
                {
                    for config_kind in [Kind::Components, Kind::Icons, Kind::Colors] {
                        let dep = theme.reference(config_kind);
                        if self.dirty(config_kind, dep) {
                            set.insert((config_kind, dep.clone()));
                        }
                    }
                }
                for op in &self.renames {
                    if op.kind == k && op.new == reference {
                        set.insert((op.kind, op.new.clone()));
                    }
                    // A renamed reference needs its resource rename in the same transaction.
                    if k == Kind::Theme
                        && let Ok(t) = self.draft.theme(&reference)
                        && op.kind != Kind::Theme
                        && *t.reference(op.kind) == op.new
                    {
                        set.insert((op.kind, op.new.clone()));
                    }
                }
            }
            for group in &self.groups {
                if !group.is_disjoint(&set) {
                    set.extend(group.iter().cloned());
                }
            }
            if set.len() == old {
                break;
            }
        }
        Ok(set)
    }

    pub fn scope_names(&self, kind: Kind, r: &ResourceRef) -> Result<Vec<String>> {
        Ok(self
            .scope(kind, r)?
            .iter()
            .filter(|(k, r)| self.dirty(*k, r))
            .map(|(k, r)| format!("{}: {}", k.label(), r.name))
            .collect())
    }

    pub fn save(
        &mut self,
        kind: Kind,
        r: &ResourceRef,
        activate: Option<ResourceRef>,
    ) -> Result<()> {
        let scope = self.scope(kind, r)?;
        let mut candidate = self.store.base.clone();
        // Patch persisted references only, retaining unrelated dependent-theme drafts.
        for op in &self.renames {
            if scope.contains(&(op.kind, op.new.clone())) {
                candidate.remove(op.kind, &op.old.name);
                candidate.rename_refs(op.kind, &op.old, &op.new);
            }
        }
        for (k, reference) in &scope {
            if reference.source == Source::Builtin {
                continue;
            }
            if let Ok(resource) = self.draft.get(*k, reference) {
                candidate.put(reference.name.clone(), resource);
            } else {
                candidate.remove(*k, &reference.name);
            }
        }
        if let Some(active) = activate {
            candidate.active_theme = active;
        }
        self.store.save(&candidate)?;
        for (k, reference) in &scope {
            if reference.source == Source::User {
                if let Ok(resource) = candidate.get(*k, reference) {
                    self.draft.put(reference.name.clone(), resource);
                } else {
                    self.draft.remove(*k, &reference.name);
                }
            }
        }
        self.draft.active_theme = candidate.active_theme;
        self.groups.retain(|g| g.is_disjoint(&scope));
        self.renames
            .retain(|op| !scope.contains(&(op.kind, op.new.clone())));
        Ok(())
    }

    pub fn save_all(&mut self, activate: Option<ResourceRef>) -> Result<()> {
        let mut candidate = self.draft.clone();
        if let Some(active) = activate {
            candidate.active_theme = active;
        }
        self.store.save(&candidate)?;
        self.draft = candidate;
        self.groups.clear();
        self.renames.clear();
        Ok(())
    }

    pub fn activate(&mut self, theme: &ResourceRef) -> Result<()> {
        self.store.base.resolve(theme)?;
        let mut catalog = self.store.base.clone();
        catalog.active_theme = theme.clone();
        self.store.save(&catalog)?;
        self.draft.active_theme = theme.clone();
        Ok(())
    }

    pub fn discard(&mut self) {
        self.draft = self.store.base.clone();
        self.groups.clear();
        self.renames.clear();
    }

    pub fn reload(&mut self) -> Result<()> {
        self.store.reload()?;
        self.discard();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn session() -> (tempfile::TempDir, Session) {
        let dir = tempfile::tempdir().unwrap();
        let s = Session::new(Store::open(dir.path()).unwrap());
        (dir, s)
    }
    #[test]
    fn duplicate_reuses_refs_full_fork_copies_all_and_saves_as_a_unit() {
        let (_dir, mut s) = session();
        let source = ResourceRef::builtin("Default");
        let linked = s.duplicate_theme(&source, "Linked", false).unwrap();
        assert_eq!(
            s.draft.theme(&linked).unwrap(),
            s.draft.theme(&source).unwrap()
        );
        let fork = s.duplicate_theme(&source, "Fork", true).unwrap();
        let t = s.draft.theme(&fork).unwrap();
        for k in [Kind::Components, Kind::Icons, Kind::Colors] {
            assert_eq!(t.reference(k).source, Source::User);
        }
        s.save(Kind::Icons, &t.icons, None).unwrap();
        assert!(s.store.base.theme(&fork).is_ok());
        assert!(s.dirty(Kind::Theme, &linked));
        assert!(!s.dirty(Kind::Theme, &fork));
        assert_eq!(s.store.base.active_theme, source);
    }
    #[test]
    fn copying_one_shared_resource_only_retargets_the_current_theme() {
        let (_dir, mut s) = session();
        let original = ResourceRef::builtin("Default");
        let first = s.duplicate_theme(&original, "One", false).unwrap();
        let second = s.duplicate_theme(&original, "Two", false).unwrap();
        s.save_all(None).unwrap();
        let (_, copy) = s.customize(&first, Kind::Colors, "My colors").unwrap();
        assert_eq!(
            s.draft.theme(&second).unwrap().colors,
            ResourceRef::builtin("Default")
        );
        s.save(Kind::Colors, &copy, None).unwrap();
        assert_eq!(s.store.base.theme(&first).unwrap().colors, copy);
        assert!(!s.any_dirty());
    }
    #[test]
    fn scoped_rename_patches_saved_refs_without_committing_other_theme_edits() {
        let (_dir, mut s) = session();
        let t = s
            .duplicate_theme(&ResourceRef::builtin("Default"), "Work", true)
            .unwrap();
        s.save_all(Some(t.clone())).unwrap();
        let colors = s.draft.theme(&t).unwrap().colors;
        s.draft.themes.get_mut(&t.name).unwrap().description =
            "Unrelated unsaved description".into();
        let new = s.rename(Kind::Colors, &colors, "Palette").unwrap();
        s.save(Kind::Colors, &new, None).unwrap();
        assert_eq!(s.store.base.theme(&t).unwrap().colors, new);
        assert_eq!(s.store.base.theme(&t).unwrap().description, "");
        assert!(s.dirty(Kind::Theme, &t));
        assert!(!s.dirty(Kind::Colors, &new));
        assert_eq!(
            s.draft.theme(&t).unwrap().description,
            "Unrelated unsaved description"
        );
    }
    #[test]
    fn invalid_unrelated_draft_does_not_block_scoped_save_and_failure_keeps_drafts() {
        let (_dir, mut s) = session();
        let good = s
            .duplicate_theme(&ResourceRef::builtin("Default"), "Good", false)
            .unwrap();
        let bad = s.duplicate_theme(&good, "Bad", false).unwrap();
        s.draft.themes.get_mut(&bad.name).unwrap().colors = ResourceRef::user("missing");
        s.save(Kind::Theme, &good, None).unwrap();
        assert!(s.dirty(Kind::Theme, &bad));
        let bytes = std::fs::read(&s.store.path).unwrap();
        assert!(s.save_all(Some(bad.clone())).is_err());
        assert_eq!(std::fs::read(&s.store.path).unwrap(), bytes);
        assert_eq!(s.store.base.active_theme, ResourceRef::builtin("Default"));
        assert!(s.dirty(Kind::Theme, &bad));
    }
    #[test]
    fn delete_blocks_dependencies_and_active_theme_and_rename_tracks_active() {
        let (_dir, mut s) = session();
        let t = s
            .duplicate_theme(&ResourceRef::builtin("Default"), "Work", true)
            .unwrap();
        s.save_all(Some(t.clone())).unwrap();
        let colors = s.draft.theme(&t).unwrap().colors;
        assert!(s.delete(Kind::Colors, &colors).is_err());
        assert!(s.delete(Kind::Theme, &t).is_err());
        let renamed = s.rename(Kind::Theme, &t, "Renamed").unwrap();
        s.save(Kind::Theme, &renamed, None).unwrap();
        assert_eq!(s.store.base.active_theme, renamed);
        s.activate(&ResourceRef::builtin("Default")).unwrap();
        s.delete(Kind::Theme, &renamed).unwrap();
        s.save(Kind::Theme, &renamed, None).unwrap();
        assert!(s.store.base.theme(&renamed).is_err());
        assert!(s.store.base.get(Kind::Colors, &colors).is_ok());
    }
    #[test]
    fn shared_edit_changes_active_resolution_without_reactivation() {
        let (_dir, mut s) = session();
        let t = s
            .duplicate_theme(&ResourceRef::builtin("Default"), "Work", true)
            .unwrap();
        s.save_all(Some(t.clone())).unwrap();
        let colors = s.draft.theme(&t).unwrap().colors;
        s.draft
            .color_configs
            .get_mut(&colors.name)
            .unwrap()
            .entry_mut("model")
            .text_bold = true;
        s.save(Kind::Colors, &colors, None).unwrap();
        let reopened = Store::open(s.store.path.parent().unwrap()).unwrap();
        assert_eq!(reopened.base.active_theme, t);
        assert!(
            reopened
                .base
                .resolve(&t)
                .unwrap()
                .get_component(super::super::types::ComponentId::Model)
                .unwrap()
                .styles
                .text_bold
        );
    }
}
