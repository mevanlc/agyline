use std::collections::{BTreeMap, HashSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::theme::ResolvedTheme;
use super::types::*;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Theme,
    Components,
    Icons,
    Colors,
}

impl Kind {
    pub const ALL: [Self; 4] = [Self::Theme, Self::Components, Self::Icons, Self::Colors];
    pub fn label(self) -> &'static str {
        match self {
            Self::Theme => "Themes",
            Self::Components => "Components",
            Self::Icons => "Icons",
            Self::Colors => "Colors",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Builtin,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceRef {
    pub source: Source,
    pub name: String,
}

impl ResourceRef {
    pub fn builtin(name: impl Into<String>) -> Self {
        Self {
            source: Source::Builtin,
            name: name.into(),
        }
    }
    pub fn user(name: impl Into<String>) -> Self {
        Self {
            source: Source::User,
            name: name.into(),
        }
    }
    pub fn label(&self) -> String {
        format!(
            "{} [{}]",
            self.name,
            if self.source == Source::Builtin {
                "Built-in"
            } else {
                "Custom"
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedTheme {
    #[serde(default)]
    pub description: String,
    pub components: ResourceRef,
    pub icons: ResourceRef,
    pub colors: ResourceRef,
}

impl NamedTheme {
    pub fn reference(&self, kind: Kind) -> &ResourceRef {
        match kind {
            Kind::Components => &self.components,
            Kind::Icons => &self.icons,
            Kind::Colors => &self.colors,
            Kind::Theme => unreachable!("themes cannot reference themes"),
        }
    }
    pub fn reference_mut(&mut self, kind: Kind) -> &mut ResourceRef {
        match kind {
            Kind::Components => &mut self.components,
            Kind::Icons => &mut self.icons,
            Kind::Colors => &mut self.colors,
            Kind::Theme => unreachable!("themes cannot reference themes"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentEntry {
    pub id: String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub options: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentProfile {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub components: Vec<ComponentEntry>,
}

impl ComponentProfile {
    pub fn completed(&self) -> Self {
        let mut result = self.clone();
        for comp in ResolvedTheme::default_theme().components {
            if comp.id == ComponentId::Separator
                || result.components.iter().any(|c| c.id == comp.id.key())
            {
                continue;
            }
            result.components.push(ComponentEntry {
                id: comp.id.key(),
                enabled: false,
                options: comp
                    .options
                    .into_iter()
                    .filter(|(k, _)| k != "thinking_icon")
                    .collect(),
            });
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlyphMode {
    Plain,
    NerdFont,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Glyphs {
    pub plain: String,
    pub nerd_font: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_model: Option<PerModelIcons>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub thinking_icon: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Separator {
    pub enabled: bool,
    pub plain: String,
    pub nerd_font: String,
    pub powerline_plain: String,
    pub powerline_nerd_font: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconProfile {
    #[serde(default)]
    pub description: String,
    pub glyph_mode: GlyphMode,
    pub powerline: bool,
    pub defaults: Glyphs,
    #[serde(default)]
    pub components: BTreeMap<String, Glyphs>,
    pub separator: Separator,
}

impl IconProfile {
    pub fn mode(&self) -> StyleMode {
        match (self.glyph_mode, self.powerline) {
            (GlyphMode::Plain, false) => StyleMode::Plain,
            (GlyphMode::NerdFont, false) => StyleMode::NerdFont,
            (GlyphMode::Plain, true) => StyleMode::PlainPowerline,
            (GlyphMode::NerdFont, true) => StyleMode::Powerline,
        }
    }
    pub fn entry(&self, id: &str) -> &Glyphs {
        self.components.get(id).unwrap_or(&self.defaults)
    }
    pub fn entry_mut(&mut self, id: &str) -> &mut Glyphs {
        self.components
            .entry(id.into())
            .or_insert_with(|| self.defaults.clone())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<AnsiColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<AnsiColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<AnsiColor>,
    #[serde(default)]
    pub text_bold: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeparatorColor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<AnsiColor>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorProfile {
    #[serde(default)]
    pub description: String,
    pub defaults: ColorStyle,
    #[serde(default)]
    pub components: BTreeMap<String, ColorStyle>,
    pub separator: SeparatorColor,
}

impl ColorProfile {
    pub fn entry(&self, id: &str) -> &ColorStyle {
        self.components.get(id).unwrap_or(&self.defaults)
    }
    pub fn entry_mut(&mut self, id: &str) -> &mut ColorStyle {
        self.components
            .entry(id.into())
            .or_insert_with(|| self.defaults.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Transient owned edit value; catalogs store each resource kind in its own map.
#[allow(clippy::large_enum_variant)]
pub enum Resource {
    Theme(NamedTheme),
    Components(ComponentProfile),
    Icons(IconProfile),
    Colors(ColorProfile),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub active_theme: ResourceRef,
    #[serde(default)]
    pub themes: BTreeMap<String, NamedTheme>,
    #[serde(default)]
    pub component_configs: BTreeMap<String, ComponentProfile>,
    #[serde(default)]
    pub icon_configs: BTreeMap<String, IconProfile>,
    #[serde(default)]
    pub color_configs: BTreeMap<String, ColorProfile>,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            schema_version: 2,
            active_theme: ResourceRef::builtin("Default"),
            themes: BTreeMap::new(),
            component_configs: BTreeMap::new(),
            icon_configs: BTreeMap::new(),
            color_configs: BTreeMap::new(),
        }
    }
}

pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 200
        && name == name.trim()
        && !name.chars().any(char::is_control)
}

impl Catalog {
    pub fn get(&self, kind: Kind, reference: &ResourceRef) -> Result<Resource> {
        let c = if reference.source == Source::Builtin {
            builtins()
        } else {
            self
        };
        let value = match kind {
            Kind::Theme => c.themes.get(&reference.name).cloned().map(Resource::Theme),
            Kind::Components => c
                .component_configs
                .get(&reference.name)
                .cloned()
                .map(Resource::Components),
            Kind::Icons => c
                .icon_configs
                .get(&reference.name)
                .cloned()
                .map(Resource::Icons),
            Kind::Colors => c
                .color_configs
                .get(&reference.name)
                .cloned()
                .map(Resource::Colors),
        };
        value.ok_or_else(|| format!("{}: missing {}", kind.label(), reference.label()))
    }

    pub fn theme(&self, reference: &ResourceRef) -> Result<NamedTheme> {
        match self.get(Kind::Theme, reference)? {
            Resource::Theme(t) => Ok(t),
            _ => unreachable!(),
        }
    }

    pub fn names(&self, kind: Kind) -> Vec<String> {
        match kind {
            Kind::Theme => self.themes.keys().cloned().collect(),
            Kind::Components => self.component_configs.keys().cloned().collect(),
            Kind::Icons => self.icon_configs.keys().cloned().collect(),
            Kind::Colors => self.color_configs.keys().cloned().collect(),
        }
    }

    pub fn list(&self, kind: Kind) -> Vec<ResourceRef> {
        builtins()
            .names(kind)
            .into_iter()
            .map(ResourceRef::builtin)
            .chain(self.names(kind).into_iter().map(ResourceRef::user))
            .collect()
    }

    pub fn put(&mut self, name: String, value: Resource) {
        match value {
            Resource::Theme(v) => {
                self.themes.insert(name, v);
            }
            Resource::Components(v) => {
                self.component_configs.insert(name, v.completed());
            }
            Resource::Icons(v) => {
                self.icon_configs.insert(name, v);
            }
            Resource::Colors(v) => {
                self.color_configs.insert(name, v);
            }
        }
    }

    pub fn remove(&mut self, kind: Kind, name: &str) {
        match kind {
            Kind::Theme => {
                self.themes.remove(name);
            }
            Kind::Components => {
                self.component_configs.remove(name);
            }
            Kind::Icons => {
                self.icon_configs.remove(name);
            }
            Kind::Colors => {
                self.color_configs.remove(name);
            }
        }
    }

    pub fn check_new_name(&self, kind: Kind, name: &str, replacing: Option<&str>) -> Result<()> {
        if !valid_name(name) {
            return Err("Names must be nonempty, trimmed, at most 200 bytes, and contain no control characters".into());
        }
        if self
            .names(kind)
            .iter()
            .any(|n| Some(n.as_str()) != replacing && n.to_lowercase() == name.to_lowercase())
        {
            return Err(format!("{}: '{}' already exists", kind.label(), name));
        }
        Ok(())
    }

    pub fn suggested_name(&self, kind: Kind, base: &str) -> String {
        let base: String = base
            .chars()
            .scan(0, |n, c| {
                *n += c.len_utf8();
                (*n <= 160).then_some(c)
            })
            .collect();
        let mut name = format!("{base} Copy");
        let mut n = 2;
        while self.check_new_name(kind, &name, None).is_err() {
            name = format!("{base} Copy {n}");
            n += 1;
        }
        name
    }

    pub fn dependents(&self, kind: Kind, reference: &ResourceRef) -> Vec<ResourceRef> {
        if kind == Kind::Theme {
            return Vec::new();
        }
        self.list(Kind::Theme)
            .into_iter()
            .filter(|r| self.theme(r).is_ok_and(|t| t.reference(kind) == reference))
            .collect()
    }

    pub fn rename_refs(&mut self, kind: Kind, old: &ResourceRef, new: &ResourceRef) {
        if kind == Kind::Theme {
            if self.active_theme == *old {
                self.active_theme = new.clone();
            }
        } else {
            for theme in self.themes.values_mut() {
                if theme.reference(kind) == old {
                    *theme.reference_mut(kind) = new.clone();
                }
            }
        }
    }

    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();
        if self.schema_version != 2 {
            return Err(format!(
                "schema_version: unsupported version {} (expected 2)",
                self.schema_version
            ));
        }
        for kind in Kind::ALL {
            let mut seen = HashSet::new();
            for name in self.names(kind) {
                if !valid_name(&name) || !seen.insert(name.to_lowercase()) {
                    errors.push(format!(
                        "{}: invalid or duplicate name '{name}'",
                        kind.label()
                    ));
                }
                let r = ResourceRef::user(&name);
                if let Err(e) = self.validate_resource(kind, &r) {
                    errors.push(e);
                }
            }
        }
        if let Err(e) = self.resolve(&self.active_theme) {
            errors.push(format!("active_theme: {e}"));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    pub fn validate_resource(&self, kind: Kind, reference: &ResourceRef) -> Result<()> {
        let prefix = format!("{} '{}'", kind.label(), reference.name);
        match self.get(kind, reference)? {
            Resource::Theme(t) => {
                for k in [Kind::Components, Kind::Icons, Kind::Colors] {
                    let r = t.reference(k);
                    if !valid_name(&r.name) {
                        return Err(format!("{prefix}: invalid reference name"));
                    }
                    self.get(k, r).map_err(|e| format!("{prefix}: {e}"))?;
                }
                self.resolve(reference)?;
            }
            Resource::Components(p) => {
                let mut ids = HashSet::new();
                for c in &p.components {
                    if c.id.is_empty()
                        || !ids.insert(&c.id)
                        || ComponentId::from_key(&c.id) == ComponentId::Separator
                    {
                        return Err(format!("{prefix}.{}: invalid/duplicate component ID", c.id));
                    }
                    validate_options(c).map_err(|e| format!("{prefix}.{}: {e}", c.id))?;
                }
            }
            Resource::Icons(p) => {
                for (id, g) in p
                    .components
                    .iter()
                    .chain(std::iter::once((&"defaults".to_string(), &p.defaults)))
                {
                    for value in [&g.plain, &g.nerd_font, &g.thinking_icon] {
                        if value.chars().any(char::is_control) {
                            return Err(format!(
                                "{prefix}.{id}: glyphs cannot contain control characters"
                            ));
                        }
                    }
                    if let Some(pm) = &g.per_model {
                        for tier in [
                            &pm.flash,
                            &pm.pro,
                            &pm.ultra,
                            &pm.flash_lite,
                            &pm.opus,
                            &pm.sonnet,
                            &pm.haiku,
                            &pm.fable,
                            &pm.mythos,
                        ] {
                            if tier
                                .plain
                                .chars()
                                .chain(tier.nerd_font.chars())
                                .any(char::is_control)
                            {
                                return Err(format!("{prefix}.{id}: invalid model glyph"));
                            }
                        }
                    }
                }
                for value in [
                    &p.separator.plain,
                    &p.separator.nerd_font,
                    &p.separator.powerline_plain,
                    &p.separator.powerline_nerd_font,
                ] {
                    if value.chars().any(char::is_control) {
                        return Err(format!("{prefix}.separator: invalid glyph"));
                    }
                }
            }
            Resource::Colors(p) => {
                for style in p.components.values().chain(std::iter::once(&p.defaults)) {
                    for color in [&style.icon, &style.text, &style.background] {
                        validate_color(color).map_err(|e| format!("{prefix}: {e}"))?;
                    }
                }
                validate_color(&p.separator.icon)
                    .map_err(|e| format!("{prefix}.separator: {e}"))?;
            }
        }
        Ok(())
    }

    pub fn resolve(&self, reference: &ResourceRef) -> Result<ResolvedTheme> {
        let t = self.theme(reference)?;
        self.resolve_definition(&t)
    }

    pub fn resolve_definition(&self, t: &NamedTheme) -> Result<ResolvedTheme> {
        let Resource::Components(layout) = self.get(Kind::Components, &t.components)? else {
            unreachable!()
        };
        let Resource::Icons(icons) = self.get(Kind::Icons, &t.icons)? else {
            unreachable!()
        };
        let Resource::Colors(colors) = self.get(Kind::Colors, &t.colors)? else {
            unreachable!()
        };
        let mut components = Vec::new();
        for c in layout.completed().components {
            let id = ComponentId::from_key(&c.id);
            if id == ComponentId::Unknown {
                if c.enabled {
                    return Err(format!(
                        "Components '{}'.{}: enabled unknown component",
                        t.components.name, c.id
                    ));
                }
                continue;
            }
            if id == ComponentId::Separator {
                return Err("Separator is not a data component".into());
            }
            let g = icons.entry(&c.id);
            let v = colors.entry(&c.id);
            let mut options: std::collections::HashMap<_, _> = c.options.into_iter().collect();
            if id == ComponentId::Model {
                options.insert("thinking_icon".into(), g.thinking_icon.clone().into());
            }
            components.push(ComponentConfig {
                id,
                enabled: c.enabled,
                options,
                icon: IconConfig {
                    plain: g.plain.clone(),
                    nerd_font: g.nerd_font.clone(),
                    per_model: g.per_model.clone(),
                },
                colors: ColorConfig {
                    icon: v.icon.clone(),
                    text: v.text.clone(),
                    background: v.background.clone(),
                },
                styles: TextStyleConfig {
                    text_bold: v.text_bold,
                },
            });
        }
        components.push(ComponentConfig {
            id: ComponentId::Separator,
            enabled: icons.separator.enabled,
            options: Default::default(),
            icon: IconConfig {
                plain: icons.separator.plain.clone(),
                nerd_font: icons.separator.nerd_font.clone(),
                per_model: None,
            },
            colors: ColorConfig {
                icon: colors.separator.icon.clone(),
                ..Default::default()
            },
            styles: TextStyleConfig::default(),
        });
        Ok(ResolvedTheme {
            style: StyleConfig { mode: icons.mode() },
            components,
            powerline_plain: icons.separator.powerline_plain.clone(),
            powerline_nerd_font: icons.separator.powerline_nerd_font.clone(),
        })
    }
}

fn validate_color(c: &Option<AnsiColor>) -> Result<()> {
    if matches!(c, Some(AnsiColor::Color16 { c16 }) if *c16 > 15) {
        Err("c16 must be between 0 and 15".into())
    } else {
        Ok(())
    }
}

fn validate_options(c: &ComponentEntry) -> Result<()> {
    for (key, value) in &c.options {
        let valid = match key.as_str() {
            "thinking_icon" => return Err("thinking_icon belongs to the icon config".into()),
            "show_sha"
            | "autohide_branch"
            | "show_original_branch"
            | "show_review_state"
            | "show_url"
            | "osc_hyperlinks" => value.is_boolean(),
            "rstrip" | "search" | "replace" => value.is_string(),
            "show_effort" => value
                .as_str()
                .is_some_and(|s| ["show", "gemini", "third_party", "hide"].contains(&s)),
            "outside_worktrees" => value
                .as_str()
                .is_some_and(|s| ["hide", "show", "branch", "directory"].contains(&s)),
            "value" => value
                .as_str()
                .is_some_and(|s| ["used", "remaining"].contains(&s)),
            _ => true,
        };
        if !valid {
            return Err(format!("options.{key}: invalid value"));
        }
    }
    Ok(())
}

pub fn profiles_from_runtime(
    theme: &ResolvedTheme,
) -> (ComponentProfile, IconProfile, ColorProfile) {
    let mut c = ComponentProfile::default();
    let mut i = IconProfile {
        description: String::new(),
        defaults: Glyphs::default(),
        components: BTreeMap::new(),
        glyph_mode: if matches!(
            theme.style.mode,
            StyleMode::Plain | StyleMode::PlainPowerline
        ) {
            GlyphMode::Plain
        } else {
            GlyphMode::NerdFont
        },
        powerline: matches!(
            theme.style.mode,
            StyleMode::Powerline | StyleMode::PlainPowerline
        ),
        separator: Separator {
            enabled: true,
            plain: " | ".into(),
            nerd_font: " | ".into(),
            powerline_plain: "►".into(),
            powerline_nerd_font: "\u{e0b0}".into(),
        },
    };
    let mut colors = ColorProfile::default();
    for comp in &theme.components {
        if comp.id == ComponentId::Separator {
            i.separator.enabled = comp.enabled;
            i.separator.plain = comp.icon.plain.clone();
            i.separator.nerd_font = comp.icon.nerd_font.clone();
            colors.separator.icon = comp.colors.icon.clone();
            continue;
        }
        let mut options: BTreeMap<_, _> = comp.options.clone().into_iter().collect();
        let thinking = options
            .remove("thinking_icon")
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap_or_default();
        c.components.push(ComponentEntry {
            id: comp.id.key(),
            enabled: comp.enabled,
            options,
        });
        i.components.insert(
            comp.id.key(),
            Glyphs {
                plain: comp.icon.plain.clone(),
                nerd_font: comp.icon.nerd_font.clone(),
                per_model: comp.icon.per_model.clone(),
                thinking_icon: thinking,
            },
        );
        colors.components.insert(
            comp.id.key(),
            ColorStyle {
                icon: comp.colors.icon.clone(),
                text: comp.colors.text.clone(),
                background: comp.colors.background.clone(),
                text_bold: comp.styles.text_bold,
            },
        );
    }
    (c, i, colors)
}

pub fn builtins() -> &'static Catalog {
    static BUILTINS: OnceLock<Catalog> = OnceLock::new();
    BUILTINS.get_or_init(|| {
        let mut c = Catalog::default();
        let default = ResolvedTheme::default_theme();
        c.component_configs
            .insert("Default".into(), profiles_from_runtime(&default).0);
        for set in crate::presets::icon_sets::all() {
            let mut t = default.clone();
            set.apply_to(&mut t.components);
            t.style.mode = match set.name {
                "Emoji" | "Minimal" => StyleMode::Plain,
                "Powerline" => StyleMode::Powerline,
                _ => StyleMode::NerdFont,
            };
            let mut profile = profiles_from_runtime(&t).1;
            profile.description = set.description.into();
            c.icon_configs.insert(set.name.into(), profile);
        }
        for scheme in crate::presets::color_schemes::all() {
            let mut t = default.clone();
            scheme.apply_to(&mut t.components);
            let mut profile = profiles_from_runtime(&t).2;
            profile.description = scheme.description.into();
            c.color_configs.insert(scheme.name.into(), profile);
        }
        for (name, icons) in [
            ("Default", "Emoji"),
            ("Cometix", "Nerd Font"),
            ("Minimal", "Minimal"),
            ("Gruvbox", "Nerd Font"),
            ("Nord", "Nerd Font"),
            ("Powerline Dark", "Powerline"),
            ("Powerline Light", "Powerline"),
            ("Rose Pine", "Nerd Font"),
            ("Tokyo Night", "Nerd Font"),
            ("Late", "Late"),
        ] {
            c.themes.insert(
                name.into(),
                NamedTheme {
                    description: String::new(),
                    components: ResourceRef::builtin("Default"),
                    icons: ResourceRef::builtin(icons),
                    colors: ResourceRef::builtin(name),
                },
            );
        }
        c
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builtins_resolve_and_match_the_palette_and_icon_seeds() {
        let c = Catalog::default();
        assert_eq!(c.list(Kind::Theme).len(), 10);
        assert_eq!(c.list(Kind::Icons).len(), 5);
        for reference in c.list(Kind::Theme) {
            let t = c.theme(&reference).unwrap();
            let actual = c.resolve(&reference).unwrap();
            let mut expected = ResolvedTheme::default_theme();
            crate::presets::icon_sets::find(&t.icons.name)
                .unwrap()
                .apply_to(&mut expected.components);
            crate::presets::color_schemes::find(&t.colors.name)
                .unwrap()
                .apply_to(&mut expected.components);
            expected
                .components
                .iter_mut()
                .find(|c| c.id == ComponentId::Model)
                .unwrap()
                .options
                .entry("thinking_icon".into())
                .or_insert_with(|| "".into());
            assert_eq!(actual.components, expected.components, "{}", reference.name);
        }
    }
    #[test]
    fn omitted_components_are_disabled_and_unknown_disabled_entries_roundtrip() {
        let mut c = Catalog::default();
        let mut p = ComponentProfile::default();
        p.components.push(ComponentEntry {
            id: "future_metric".into(),
            enabled: false,
            options: BTreeMap::from([("future_option".into(), "keep me".into())]),
        });
        c.put("Layout".into(), Resource::Components(p));
        let mut t = c.theme(&c.active_theme).unwrap();
        t.components = ResourceRef::user("Layout");
        c.put("Work".into(), Resource::Theme(t));
        c.active_theme = ResourceRef::user("Work");
        c.validate().unwrap();
        let resolved = c.resolve(&c.active_theme).unwrap();
        assert!(
            resolved
                .components
                .iter()
                .filter(|v| v.id != ComponentId::Separator)
                .all(|v| !v.enabled)
        );
        let text = toml::to_string(&c).unwrap();
        assert_eq!(toml::from_str::<Catalog>(&text).unwrap(), c);
        c.component_configs.get_mut("Layout").unwrap().components[0].enabled = true;
        assert!(c.validate().unwrap_err().contains("enabled unknown"));
    }
    #[test]
    fn fallback_colors_and_empty_multichar_glyphs_resolve_independently() {
        let mut c = Catalog::default();
        let mut t = c.theme(&c.active_theme).unwrap();
        let Resource::Icons(mut icons) = c.get(Kind::Icons, &t.icons).unwrap() else {
            panic!()
        };
        icons.components.clear();
        icons.defaults.plain = "👩‍💻 ok".into();
        icons.defaults.nerd_font = "".into();
        let colors = ColorProfile {
            defaults: ColorStyle {
                text_bold: true,
                text: Some(AnsiColor::Color256 { c256: 100 }),
                ..Default::default()
            },
            ..Default::default()
        };
        c.put("Glyphs".into(), Resource::Icons(icons));
        c.put("Colors".into(), Resource::Colors(colors));
        t.icons = ResourceRef::user("Glyphs");
        t.colors = ResourceRef::user("Colors");
        c.put("Work".into(), Resource::Theme(t));
        c.validate().unwrap();
        let resolved = c.resolve(&ResourceRef::user("Work")).unwrap();
        let model = resolved.get_component(ComponentId::Model).unwrap();
        assert_eq!(model.icon.plain, "👩‍💻 ok");
        assert_eq!(model.icon.nerd_font, "");
        assert!(model.styles.text_bold);
        c.icon_configs.get_mut("Glyphs").unwrap().defaults.plain = "\x1b[31m".into();
        assert!(c.validate().is_err());
    }
    #[test]
    fn strict_schema_rejects_unknown_fields_versions_and_bad_options() {
        assert!(
            toml::from_str::<Catalog>(
                "schema_version=2\nextra=1\n[active_theme]\nsource='builtin'\nname='Default'"
            )
            .is_err()
        );
        let mut c = Catalog {
            schema_version: 3,
            ..Default::default()
        };
        assert!(c.validate().is_err());
        c.schema_version = 2;
        let mut p = profiles_from_runtime(&ResolvedTheme::default_theme()).0;
        p.components[0]
            .options
            .insert("autohide_branch".into(), "yes".into());
        c.put("Bad".into(), Resource::Components(p));
        assert!(
            c.validate()
                .unwrap_err()
                .contains("options.autohide_branch")
        );
    }
}
