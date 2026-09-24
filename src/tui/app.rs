use super::{
    editor_state::{EditorModal, EditorState, Panel},
    layout::{self, Geometry},
    widgets::{
        self,
        editor::{EditorWidget, FieldSelection},
    },
};
use crate::config::{catalog::*, session::Session, store::Store, theme::ResolvedTheme, types::*};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use ratatui_textarea::{CursorMove, TextArea};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
enum Action {
    Save,
    SaveAll,
    Activate,
    SaveActivateExit,
    Duplicate,
    FullFork,
    Customize,
    Shared,
    Defaults,
    Rename,
    Delete,
    Choose,
    Dependents,
    Reload,
    Export,
    Help,
    Exit,
}
const ACTIONS: &[(Action, &str, char)] = &[
    (Action::Save, "Save current context", 's'),
    (Action::SaveAll, "Save all drafts", 'S'),
    (Action::Activate, "Activate current theme", 'a'),
    (Action::SaveActivateExit, "Save, activate and exit", 'x'),
    (Action::Duplicate, "Duplicate (reuse theme references)", 'd'),
    (Action::FullFork, "Full fork of current theme", 'f'),
    (Action::Customize, "Customize a copy for this theme", 'c'),
    (Action::Shared, "Edit shared original explicitly", 'e'),
    (Action::Defaults, "Use defaults for selected component", 'v'),
    (Action::Rename, "Rename custom resource", 'r'),
    (Action::Delete, "Delete custom resource", 'D'),
    (Action::Choose, "Choose a resource", '/'),
    (Action::Dependents, "Inspect dependent themes", 'u'),
    (Action::Reload, "Reload catalog / discard drafts", 'R'),
    (Action::Export, "Export recovery draft catalog", 'E'),
    (Action::Help, "Help and shortcuts", '?'),
    (Action::Exit, "Exit", 'q'),
];
#[derive(Clone, Copy)]
enum Naming {
    Duplicate,
    FullFork,
    Customize,
    Rename,
}
#[derive(Clone)]
enum Confirm {
    Save {
        all: bool,
        activate: bool,
        exit: bool,
    },
    Shared,
    Rename,
    Delete,
    Reload,
    Exit,
}
enum Modal {
    Menu {
        index: usize,
    },
    Choose {
        kind: Kind,
        associate: bool,
        query: TextArea<'static>,
        index: usize,
    },
    Name {
        purpose: Naming,
        input: TextArea<'static>,
        pending: Option<KeyEvent>,
        error: Option<String>,
    },
    Confirm {
        action: Confirm,
        body: String,
        scroll: u16,
    },
    Message {
        title: String,
        body: String,
        scroll: u16,
    },
}

pub struct App {
    pub session: Session,
    pub kind: Kind,
    pub context: ResourceRef,
    selected: BTreeMap<Kind, ResourceRef>,
    pub editor: EditorState,
    positions: BTreeMap<(Kind, ResourceRef), (usize, FieldSelection, Panel)>,
    shared: BTreeSet<(Kind, ResourceRef)>,
    association: usize,
    modal: Option<Modal>,
    status: String,
    pub should_quit: bool,
    viewport: usize,
    modal_viewport: usize,
}
impl App {
    pub fn new(store: Store) -> Self {
        widgets::banner::prepare_metadata();
        let context = store.base.active_theme.clone();
        let editor = EditorState::new(
            store
                .base
                .resolve(&context)
                .expect("validated active theme"),
        );
        let mut app = Self {
            session: Session::new(store),
            kind: Kind::Theme,
            context: context.clone(),
            selected: BTreeMap::from([(Kind::Theme, context)]),
            editor,
            positions: BTreeMap::new(),
            shared: BTreeSet::new(),
            association: 0,
            modal: None,
            status: "Browse freely; Save commits drafts. ? help  m actions".into(),
            should_quit: false,
            viewport: 1,
            modal_viewport: 1,
        };
        app.select_theme_refs();
        app.refresh();
        app
    }
    fn current(&self) -> ResourceRef {
        self.selected[&self.kind].clone()
    }
    fn select_theme_refs(&mut self) {
        if let Ok(t) = self.session.draft.theme(&self.context) {
            for k in [Kind::Components, Kind::Icons, Kind::Colors] {
                self.selected.insert(k, t.reference(k).clone());
            }
        }
        self.selected.insert(Kind::Theme, self.context.clone());
    }
    fn remember(&mut self) {
        self.positions.insert(
            (self.kind, self.current()),
            (
                self.editor.selected_component,
                self.editor.selected_field,
                *self.editor.selected_panel.current(),
            ),
        );
    }
    fn preview_definition(&self) -> Result<NamedTheme> {
        let mut t = self.session.draft.theme(&self.context)?;
        if self.kind != Kind::Theme {
            *t.reference_mut(self.kind) = self.current();
        }
        if let Some(Modal::Choose {
            kind,
            associate,
            query,
            index,
        }) = &self.modal
        {
            let entries = self.choices(*kind, query);
            if let Some(r) = entries.get(*index) {
                if *kind == Kind::Theme {
                    t = self.session.draft.theme(r)?;
                } else {
                    if *associate {
                        t = self.session.draft.theme(&self.context)?;
                    }
                    *t.reference_mut(*kind) = r.clone();
                }
            }
        }
        Ok(t)
    }
    fn preview(&self) -> Result<ResolvedTheme> {
        self.session
            .draft
            .resolve_definition(&self.preview_definition()?)
    }
    fn refresh(&mut self) {
        let mut theme = match self.preview() {
            Ok(t) => t,
            Err(e) => {
                self.editor.theme.components.clear();
                self.status = e;
                return;
            }
        };
        if self.kind == Kind::Components {
            theme.components.retain(|c| c.id != ComponentId::Separator);
        }
        if matches!(self.kind, Kind::Icons | Kind::Colors) {
            let mut default = ComponentConfig {
                id: ComponentId::Unknown,
                enabled: false,
                icon: IconConfig::default(),
                colors: ColorConfig::default(),
                styles: TextStyleConfig::default(),
                options: Default::default(),
            };
            match self.session.draft.get(self.kind, &self.current()) {
                Ok(Resource::Icons(p)) => {
                    default.icon = IconConfig {
                        plain: p.defaults.plain,
                        nerd_font: p.defaults.nerd_font,
                        per_model: p.defaults.per_model,
                    };
                }
                Ok(Resource::Colors(p)) => {
                    default.colors = ColorConfig {
                        icon: p.defaults.icon,
                        text: p.defaults.text,
                        background: p.defaults.background,
                    };
                    default.styles.text_bold = p.defaults.text_bold;
                }
                _ => {}
            }
            let separator = theme.components.pop().expect("resolved separator");
            theme.components.insert(0, default);
            theme.components.insert(0, separator);
        }
        self.editor.theme = theme;
        self.editor.kind = self.kind;
        if let Some((index, field, panel)) = self.positions.get(&(self.kind, self.current())) {
            self.editor.selected_component = *index;
            self.editor.selected_field = *field;
            self.editor.selected_panel.set(panel);
        } else {
            self.editor.selected_component = 0;
            self.editor.selected_field = self
                .editor
                .fields()
                .first()
                .copied()
                .unwrap_or(FieldSelection::Enabled);
            self.editor.selected_panel.set(&Panel::ComponentList);
        }
        self.editor.selected_component = self
            .editor
            .selected_component
            .min(self.editor.theme.components.len().saturating_sub(1));
        self.editor.clamp_field();
    }
    fn select(&mut self, kind: Kind, r: ResourceRef) {
        self.remember();
        self.selected.insert(kind, r.clone());
        if kind == Kind::Theme {
            self.context = r;
            self.select_theme_refs();
        }
        self.refresh();
    }
    fn sync_editor(&mut self) {
        if !self.editor.changed {
            return;
        }
        self.editor.changed = false;
        let r = self.current();
        if r.source == Source::Builtin {
            return;
        }
        let Some(c) = self
            .editor
            .theme
            .components
            .get(self.editor.selected_component)
        else {
            return;
        };
        match self.session.draft.get(self.kind, &r) {
            Ok(Resource::Components(mut p)) => {
                let (mut layout, _, _) = profiles_from_runtime(&self.editor.theme);
                layout.components.extend(
                    p.components
                        .into_iter()
                        .filter(|c| ComponentId::from_key(&c.id) == ComponentId::Unknown),
                );
                p.components = layout.components;
                self.session.draft.put(r.name, Resource::Components(p));
            }
            Ok(Resource::Icons(mut p)) => {
                if c.id == ComponentId::Separator {
                    p.glyph_mode = if matches!(
                        self.editor.theme.style.mode,
                        StyleMode::Plain | StyleMode::PlainPowerline
                    ) {
                        GlyphMode::Plain
                    } else {
                        GlyphMode::NerdFont
                    };
                    p.powerline = matches!(
                        self.editor.theme.style.mode,
                        StyleMode::Powerline | StyleMode::PlainPowerline
                    );
                    p.separator = Separator {
                        enabled: c.enabled,
                        plain: c.icon.plain.clone(),
                        nerd_font: c.icon.nerd_font.clone(),
                        powerline_plain: self.editor.theme.powerline_plain.clone(),
                        powerline_nerd_font: self.editor.theme.powerline_nerd_font.clone(),
                    };
                } else {
                    let entry = if c.id == ComponentId::Unknown {
                        &mut p.defaults
                    } else {
                        p.entry_mut(&c.id.key())
                    };
                    entry.plain = c.icon.plain.clone();
                    entry.nerd_font = c.icon.nerd_font.clone();
                    entry.per_model = c.icon.per_model.clone();
                    if c.id == ComponentId::Model {
                        entry.thinking_icon = c
                            .options
                            .get("thinking_icon")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .into();
                    }
                }
                self.session.draft.put(r.name, Resource::Icons(p));
            }
            Ok(Resource::Colors(mut p)) => {
                if c.id == ComponentId::Separator {
                    p.separator.icon = c.colors.icon.clone();
                } else {
                    let entry = if c.id == ComponentId::Unknown {
                        &mut p.defaults
                    } else {
                        p.entry_mut(&c.id.key())
                    };
                    *entry = ColorStyle {
                        icon: c.colors.icon.clone(),
                        text: c.colors.text.clone(),
                        background: c.colors.background.clone(),
                        text_bold: c.styles.text_bold,
                    };
                }
                self.session.draft.put(r.name, Resource::Colors(p));
            }
            _ => {}
        }
        self.remember();
        self.refresh();
        self.status = self
            .editor
            .status_message
            .take()
            .unwrap_or_else(|| "Draft updated".into());
    }
    fn naming(&mut self, purpose: Naming, pending: Option<KeyEvent>) {
        let r = if matches!(purpose, Naming::FullFork) {
            self.context.clone()
        } else {
            self.current()
        };
        let kind = if matches!(purpose, Naming::FullFork) {
            Kind::Theme
        } else {
            self.kind
        };
        let name = if matches!(purpose, Naming::Rename) {
            r.name
        } else {
            self.session.draft.suggested_name(kind, &r.name)
        };
        let mut input = TextArea::new(vec![name]);
        input.move_cursor(CursorMove::End);
        self.modal = Some(Modal::Name {
            purpose,
            input,
            pending,
            error: None,
        });
    }
    fn ready_to_edit(&mut self, key: KeyEvent) -> bool {
        let r = self.current();
        let deps = self.session.draft.dependents(self.kind, &r);
        if r.source == Source::Builtin
            || (!self.shared.contains(&(self.kind, r.clone()))
                && (deps.len() > 1 || (deps.len() == 1 && deps[0] != self.context)))
        {
            self.naming(Naming::Customize, Some(key));
            false
        } else {
            true
        }
    }
    fn choices(&self, kind: Kind, query: &TextArea<'_>) -> Vec<ResourceRef> {
        let q = query.lines().join("").to_lowercase();
        self.session
            .draft
            .list(kind)
            .into_iter()
            .filter(|r| r.label().to_lowercase().contains(&q))
            .collect()
    }
    fn choose(&mut self, kind: Kind, associate: bool) {
        self.modal = Some(Modal::Choose {
            kind,
            associate,
            query: TextArea::default(),
            index: 0,
        });
    }
    fn message(&mut self, title: &str, body: String) {
        self.modal = Some(Modal::Message {
            title: title.into(),
            body,
            scroll: 0,
        });
    }
    fn save_dialog(&mut self, all: bool, activate: bool, exit: bool) {
        let (kind, r) = if activate {
            (Kind::Theme, self.context.clone())
        } else {
            (self.kind, self.current())
        };
        let names = if all {
            Kind::ALL
                .into_iter()
                .flat_map(|k| {
                    self.session
                        .draft
                        .list(k)
                        .into_iter()
                        .filter(move |r| r.source == Source::User)
                        .map(move |r| (k, r))
                })
                .filter(|(k, r)| self.session.dirty(*k, r))
                .map(|(k, r)| format!("{}: {}", k.label(), r.name))
                .collect::<Vec<_>>()
        } else {
            self.session.scope_names(kind, &r).unwrap_or_default()
        };
        let body = format!(
            "Commit {}:\n{}\n\nSaved edits used by the active theme affect the next statusline.{}",
            if all {
                "all drafts"
            } else {
                "this context and required drafts"
            },
            if names.is_empty() {
                "No resource changes".into()
            } else {
                names.join("\n")
            },
            if activate {
                format!("\nActivate: {}", self.context.label())
            } else {
                String::new()
            }
        );
        self.modal = Some(Modal::Confirm {
            action: Confirm::Save {
                all,
                activate,
                exit,
            },
            body,
            scroll: 0,
        });
    }
    fn action(&mut self, action: Action) {
        match action {
            Action::Save => self.save_dialog(false, false, false),
            Action::SaveAll => self.save_dialog(true, false, false),
            Action::SaveActivateExit => self.save_dialog(false, true, true),
            Action::Activate => {
                if !self.session.scope_names(Kind::Theme, &self.context).unwrap_or_default().is_empty() {
                    self.save_dialog(false, true, false);
                } else {
                    match self.session.activate(&self.context) {
                        Ok(()) => self.status = "Active theme updated".into(),
                        Err(e) => self.message("Activation failed", e),
                    }
                }
            }
            Action::Duplicate => self.naming(Naming::Duplicate, None),
            Action::FullFork => self.naming(Naming::FullFork, None),
            Action::Customize => {
                let purpose = if self.kind == Kind::Theme { Naming::Duplicate } else { Naming::Customize };
                self.naming(purpose, None);
            }
            Action::Defaults => {
                if !matches!(self.kind, Kind::Icons | Kind::Colors) {
                    self.message("Defaults", "Choose a component in Icons or Colors first.".into());
                    return;
                }
                let Some(id) = self.editor.theme.components.get(self.editor.selected_component).map(|c| c.id) else { return; };
                if matches!(id, ComponentId::Unknown | ComponentId::Separator) {
                    self.message("Defaults", "Choose a data component first.".into());
                    return;
                }
                if !self.ready_to_edit(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE)) { return; }
                let r = self.current();
                match self.session.draft.get(self.kind, &r) {
                    Ok(Resource::Icons(mut p)) => { p.components.remove(&id.key()); self.session.draft.put(r.name, Resource::Icons(p)); }
                    Ok(Resource::Colors(mut p)) => { p.components.remove(&id.key()); self.session.draft.put(r.name, Resource::Colors(p)); }
                    _ => {}
                }
                self.remember(); self.refresh();
                self.status = "Component now uses this config's defaults".into();
            }
            Action::Rename => {
                if self.current().source == Source::Builtin {
                    self.message("Read-only", "Duplicate or customize this built-in first.".into());
                } else {
                    self.modal = Some(Modal::Confirm {
                        action: Confirm::Rename,
                        body: format!("Renaming updates all saved references when you Save. Unrelated dependent drafts stay unsaved.\n\nAffected themes:\n{}", self.dependent_text()),
                        scroll: 0,
                    });
                }
            }
            Action::Shared => {
                if self.kind == Kind::Theme || self.current().source == Source::Builtin {
                    self.message("Read-only", "Choose a custom config to edit its shared original.".into());
                    return;
                }
                self.modal = Some(Modal::Confirm {
                    action: Confirm::Shared,
                    body: format!("Edit the shared original? Saving it changes every dependent theme, including active output when listed.\n\n{}", self.dependent_text()),
                    scroll: 0,
                });
            }
            Action::Delete => self.modal = Some(Modal::Confirm {
                action: Confirm::Delete,
                body: format!("Delete {}? This commits the deletion.\n\n{}", self.current().label(), self.dependent_text()),
                scroll: 0,
            }),
            Action::Choose => self.choose(self.kind, false),
            Action::Dependents => self.message("Dependent themes", self.dependent_text()),
            Action::Reload => self.modal = Some(Modal::Confirm {
                action: Confirm::Reload,
                body: "Reload config.toml and discard all drafts? Export drafts first if you need a recovery copy.".into(),
                scroll: 0,
            }),
            Action::Export => match self.session.store.export_drafts(&self.session.draft) {
                Ok(p) => self.message("Draft catalog exported", p.display().to_string()),
                Err(e) => self.message("Export failed", e),
            },
            Action::Help => {
                let mut body = concat!(
                    "1 Themes  2 Components  3 Icons  4 Colors\n",
                    "[ / ] previous / next resource; / searchable chooser\n",
                    "Tab or ←→ switch list / inspector\n",
                    "↑↓ select; PgUp/PgDn page; Home/End jump\n",
                    "Enter/Space edit; Shift+↑↓ reorder components\n",
                    "Colors: Enter picker; Delete clear selected color\n",
                    "Icons: mode and separators are the first row\n",
                    "Defaults apply to components without overrides. v removes an override.\n",
                    "[B] built-in; [C] custom; A active; * unsaved; Shared explicit shared editing.\n",
                    "Association picker: Enter accepts; Esc restores preview.\n",
                    "Browsing a config library previews it without changing theme associations.\n",
                    "Navigation retains drafts. Built-ins and shared configs default to a named copy for this theme.\n",
                    "Ctrl+C immediately exits and discards unsaved drafts.\n\nActions (m opens menu):\n",
                ).to_string();
                body.push_str(&ACTIONS.iter().map(|(_,label,key)| format!("{key}  {label}")).collect::<Vec<_>>().join("\n"));
                self.message("Help", body);
            }
            Action::Exit => self.exit(),
        }
    }
    fn dependent_text(&self) -> String {
        let deps = self.session.draft.dependents(self.kind, &self.current());
        if deps.is_empty() {
            "No dependent themes.".into()
        } else {
            deps.iter()
                .map(|r| {
                    format!(
                        "{}{}",
                        r.label(),
                        if *r == self.session.store.base.active_theme {
                            " [Active]"
                        } else {
                            ""
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
    fn exit(&mut self) {
        if self.session.any_dirty() {
            self.modal=Some(Modal::Confirm {action:Confirm::Exit,body:"Unsaved drafts remain.\n\ns: Save all and exit\nd: Discard remaining drafts and exit\nEsc: Cancel".into(),scroll:0})
        } else {
            self.should_quit = true;
        }
    }
    fn confirm(&mut self, action: Confirm) -> Result<()> {
        match action {
            Confirm::Save {
                all,
                activate,
                exit,
            } => {
                let target = activate.then(|| self.context.clone());
                if all {
                    self.session.save_all(target)?;
                } else {
                    let (kind, r) = if activate {
                        (Kind::Theme, self.context.clone())
                    } else {
                        (self.kind, self.current())
                    };
                    self.session.save(kind, &r, target)?;
                }
                self.status = "Saved. Active output uses the saved references.".into();
                if exit {
                    self.exit();
                }
            }
            Confirm::Rename => self.naming(Naming::Rename, None),
            Confirm::Shared => {
                self.shared.insert((self.kind, self.current()));
                self.status = "Editing shared original; saving affects dependents".into();
            }
            Confirm::Delete => {
                let r = self.current();
                self.session.delete(self.kind, &r)?;
                self.session.save(self.kind, &r, None)?;
                self.select(self.kind, self.session.draft.list(self.kind)[0].clone());
                self.status = "Deleted".into();
            }
            Confirm::Reload => {
                self.session.reload()?;
                self.context = self.session.draft.active_theme.clone();
                self.select_theme_refs();
                self.shared.clear();
                self.refresh();
                self.status = "Reloaded catalog".into();
            }
            Confirm::Exit => {}
        }
        Ok(())
    }
    fn submit_name(&mut self, purpose: Naming, name: &str) -> Result<()> {
        let r = self.current();
        match purpose {
            Naming::Duplicate => {
                let new = if self.kind == Kind::Theme {
                    self.session.duplicate_theme(&r, name, false)?
                } else {
                    self.session.copy(self.kind, &r, name)?
                };
                self.select(self.kind, new);
            }
            Naming::FullFork => {
                let new = self.session.duplicate_theme(&self.context, name, true)?;
                self.remember();
                self.kind = Kind::Theme;
                self.select(Kind::Theme, new);
            }
            Naming::Customize => {
                let position = (
                    self.editor.selected_component,
                    self.editor.selected_field,
                    *self.editor.selected_panel.current(),
                );
                let (t, new) = self
                    .session
                    .customize_from(&self.context, self.kind, &r, name)?;
                self.positions.insert((self.kind, new.clone()), position);
                self.context = t;
                self.select_theme_refs();
                self.select(self.kind, new);
            }
            Naming::Rename => {
                let new = self.session.rename(self.kind, &r, name)?;
                if self.shared.remove(&(self.kind, r.clone())) {
                    self.shared.insert((self.kind, new.clone()));
                }
                if self.kind == Kind::Theme {
                    self.context = new.clone();
                } else {
                    self.select_theme_refs();
                }
                self.select(self.kind, new);
            }
        }
        self.status = "Draft created; Save commits it".into();
        Ok(())
    }
    pub fn handle_key(&mut self, code: KeyCode, mods: KeyModifiers) {
        if code == KeyCode::Char('c') && mods.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }
        if self.editor.has_modal() {
            self.editor.handle_key(code, mods);
            self.sync_editor();
            return;
        }
        if let Some(modal) = self.modal.take() {
            self.modal_key(modal, code, mods);
            return;
        }
        if let KeyCode::Char(c @ '1'..='4') = code {
            self.remember();
            let next = Kind::ALL[c as usize - '1' as usize];
            if self.kind == Kind::Theme
                && next != Kind::Theme
                && let Ok(theme) = self.session.draft.theme(&self.context)
            {
                self.selected.insert(next, theme.reference(next).clone());
            }
            self.kind = next;
            self.refresh();
            return;
        }
        match code {
            KeyCode::Char('m') => self.modal = Some(Modal::Menu { index: 0 }),
            KeyCode::Char('[' | ']') => {
                let refs = self.session.draft.list(self.kind);
                let i = refs.iter().position(|r| *r == self.current()).unwrap_or(0);
                let next = if code == KeyCode::Char(']') {
                    (i + 1) % refs.len()
                } else {
                    (i + refs.len() - 1) % refs.len()
                };
                self.select(self.kind, refs[next].clone());
            }
            KeyCode::Char(c) if ACTIONS.iter().any(|(_, _, key)| *key == c) => {
                let a = ACTIONS.iter().find(|(_, _, key)| *key == c).unwrap().0;
                self.action(a);
            }
            KeyCode::Esc => self.exit(),
            _ if self.kind == Kind::Theme => match code {
                KeyCode::Up => self.association = self.association.saturating_sub(1),
                KeyCode::Down => self.association = (self.association + 1).min(2),
                KeyCode::Tab | KeyCode::BackTab | KeyCode::Left | KeyCode::Right => {
                    self.editor.selected_panel.move_next();
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.choose(Kind::ALL[self.association + 1], true)
                }
                _ => {}
            },
            _ => {
                let edit = matches!(code, KeyCode::Enter | KeyCode::Char(' '))
                    && (self.kind == Kind::Components
                        || self.editor.selected_panel == Panel::Editor)
                    || (matches!(code, KeyCode::Up | KeyCode::Down)
                        && mods.contains(KeyModifiers::SHIFT)
                        && self.kind == Kind::Components
                        && self.editor.selected_panel == Panel::ComponentList)
                    || (code == KeyCode::Delete
                        && self.kind == Kind::Colors
                        && self.editor.selected_panel == Panel::Editor
                        && matches!(
                            self.editor.selected_field,
                            FieldSelection::IconColor
                                | FieldSelection::TextColor
                                | FieldSelection::BackgroundColor
                        ));
                if edit && !self.ready_to_edit(KeyEvent::new(code, mods)) {
                    return;
                }
                if code == KeyCode::Delete
                    && self.kind == Kind::Colors
                    && self.editor.selected_panel == Panel::Editor
                {
                    if let Some(c) = self
                        .editor
                        .theme
                        .components
                        .get_mut(self.editor.selected_component)
                    {
                        match self.editor.selected_field {
                            FieldSelection::IconColor => c.colors.icon = None,
                            FieldSelection::TextColor => c.colors.text = None,
                            FieldSelection::BackgroundColor => c.colors.background = None,
                            _ => return,
                        };
                        self.editor.changed = true;
                    }
                } else if matches!(
                    code,
                    KeyCode::PageUp | KeyCode::PageDown | KeyCode::Home | KeyCode::End
                ) {
                    let n = if matches!(code, KeyCode::Home | KeyCode::End) {
                        1000
                    } else {
                        self.viewport
                    };
                    let direction = if matches!(code, KeyCode::PageUp | KeyCode::Home) {
                        KeyCode::Up
                    } else {
                        KeyCode::Down
                    };
                    for _ in 0..n {
                        self.editor.handle_key(direction, KeyModifiers::NONE);
                    }
                } else {
                    self.editor.handle_key(code, mods);
                }
                self.sync_editor();
            }
        }
    }
    fn modal_key(&mut self, mut modal: Modal, code: KeyCode, mods: KeyModifiers) {
        if code == KeyCode::Esc {
            return;
        }
        match &mut modal {
            Modal::Menu { index } => match code {
                KeyCode::Up => *index = index.saturating_sub(1),
                KeyCode::Down => *index = (*index + 1).min(ACTIONS.len() - 1),
                KeyCode::PageUp => *index = index.saturating_sub(self.modal_viewport),
                KeyCode::PageDown => *index = (*index + self.modal_viewport).min(ACTIONS.len() - 1),
                KeyCode::Enter => {
                    self.action(ACTIONS[*index].0);
                    return;
                }
                KeyCode::Char(c) => {
                    if let Some((a, _, _)) = ACTIONS.iter().find(|(_, _, k)| *k == c) {
                        self.action(*a);
                        return;
                    }
                }
                _ => {}
            },
            Modal::Choose {
                kind,
                associate,
                query,
                index,
            } => {
                let entries = self.choices(*kind, query);
                let last = entries.len().saturating_sub(1);
                match code {
                    KeyCode::Up => *index = index.saturating_sub(1),
                    KeyCode::Down => *index = (*index + 1).min(last),
                    KeyCode::Home => *index = 0,
                    KeyCode::End => *index = last,
                    KeyCode::PageUp => *index = index.saturating_sub(self.modal_viewport),
                    KeyCode::PageDown => *index = (*index + self.modal_viewport).min(last),
                    KeyCode::Enter => {
                        if let Some(r) = entries.get(*index) {
                            if *associate {
                                match self.session.associate(&self.context, *kind, r.clone()) {
                                    Ok(t) => {
                                        self.context = t;
                                        self.select_theme_refs();
                                        self.refresh();
                                    }
                                    Err(e) => self.message("Association failed", e),
                                }
                            } else {
                                self.select(*kind, r.clone());
                            }
                        }
                        return;
                    }
                    _ => {
                        query.input(KeyEvent::new(code, mods));
                        *index = 0;
                    }
                }
            }
            Modal::Name {
                purpose,
                input,
                pending,
                error,
            } => {
                if code == KeyCode::Enter {
                    match self.submit_name(*purpose, &input.lines().join("")) {
                        Ok(()) => {
                            if let Some(key) = pending.take() {
                                self.handle_key(key.code, key.modifiers);
                            }
                            return;
                        }
                        Err(e) => *error = Some(e),
                    }
                } else {
                    input.input(KeyEvent::new(code, mods));
                }
            }
            Modal::Confirm { action, scroll, .. } => {
                if matches!(action, Confirm::Exit) {
                    match code {
                        KeyCode::Char('s') => {
                            if let Err(e) = self.session.save_all(None) {
                                self.message("Save failed; drafts retained", e)
                            } else {
                                self.should_quit = true;
                            }
                            return;
                        }
                        KeyCode::Char('d') => {
                            self.should_quit = true;
                            return;
                        }
                        _ => {}
                    }
                } else if matches!(code, KeyCode::Enter | KeyCode::Char('y')) {
                    if let Err(e) = self.confirm(action.clone()) {
                        self.message("Action failed; drafts retained", e);
                    }
                    return;
                }
                if code == KeyCode::Char('n') {
                    return;
                }
                scroll_key(scroll, code, self.modal_viewport);
            }
            Modal::Message { scroll, .. } => {
                if code == KeyCode::Enter {
                    return;
                }
                scroll_key(scroll, code, self.modal_viewport);
            }
        }
        self.modal = Some(modal);
    }
    pub fn ui(&mut self, f: &mut Frame) {
        let area = f.area();
        if area.width < 30 || area.height < 10 {
            f.render_widget(
                Paragraph::new(
                    "Terminal too small. Resize to 60×18.\nDrafts retained; Ctrl+C exits.",
                )
                .wrap(Wrap { trim: false }),
                area,
            );
            return;
        }
        let geo = Geometry::new(area, self.editor.selected_panel == Panel::Editor);
        self.viewport = geo.body.height.saturating_sub(2).max(1) as usize;
        self.editor.icon_viewport = widgets::icon_picker::viewport(area).max(1);
        if self.editor.modal == Some(EditorModal::Icon) {
            self.editor.adjust_icon_picker_scroll();
        }
        let active = &self.session.store.base.active_theme;
        let half = area.width as usize / 2;
        f.render_widget(
            Paragraph::new(format!(
                "Active:{}  Editing:{}",
                self.resource_label(Kind::Theme, active, half.saturating_sub(9)),
                self.resource_label(Kind::Theme, &self.context, half.saturating_sub(10))
            )),
            geo.context,
        );
        f.render_widget(
            Paragraph::new(Line::from(
                Kind::ALL
                    .iter()
                    .enumerate()
                    .map(|(i, k)| {
                        Span::styled(
                            if *k == self.kind {
                                format!("[{} {}]", i + 1, k.label())
                            } else {
                                format!(" {} {} ", i + 1, k.label())
                            },
                            Style::default().fg(if *k == self.kind {
                                Color::Yellow
                            } else {
                                Color::Gray
                            }),
                        )
                    })
                    .collect::<Vec<_>>(),
            )),
            geo.tabs,
        );
        self.resource_strip(f, geo.resources);
        if self.kind == Kind::Theme {
            self.theme_body(f, &geo);
        } else {
            self.component_list(f, geo.list);
            if geo.inspector.width > 0 {
                EditorWidget::render(
                    f,
                    geo.inspector,
                    &self.editor.theme,
                    self.editor.selected_component,
                    self.editor.selected_panel == Panel::Editor,
                    self.editor.selected_field,
                    self.kind,
                );
            }
        }
        match self.preview() {
            Ok(theme) => {
                if geo.preview.height == 11 {
                    widgets::banner::render(f, geo.preview, &theme);
                } else {
                    let block = if geo.preview.height >= 3 {
                        Block::bordered().title(" Draft preview ")
                    } else {
                        Block::default()
                    };
                    let inner = block.inner(geo.preview);
                    let line = Line::from(crate::core::render::render_spans(
                        &crate::core::render::demo_line(&theme),
                    ));
                    f.render_widget(
                        Paragraph::new(layout::clip_line(line, inner.width as usize)).block(block),
                        geo.preview,
                    );
                }
            }
            Err(e) => f.render_widget(
                Paragraph::new(layout::clip(&e, geo.preview.width as usize))
                    .style(Style::default().fg(Color::Red)),
                geo.preview,
            ),
        }
        f.render_widget(
            Paragraph::new(layout::clip(&self.status, area.width as usize)),
            geo.status,
        );
        let help = if self.kind == Kind::Theme {
            "Enter assign  / choose  s save  a active  m actions  ? help"
        } else {
            "Tab pane  Enter edit  / choose  s save  m actions  ? help"
        };
        f.render_widget(
            Paragraph::new(help).style(Style::default().fg(Color::Cyan)),
            geo.help,
        );
        self.draw_modal(f);
        if self.editor.modal == Some(EditorModal::Icon) {
            widgets::icon_picker::render(
                f,
                area,
                &self.editor.icon_picker,
                &self.editor.icon_catalog,
            );
        }
        if self.editor.modal == Some(EditorModal::Color) {
            widgets::color_picker::render(f, area, &self.editor.color_picker);
        }
        if self.editor.modal == Some(EditorModal::Name) {
            widgets::name_input::render(f, area, "Edit value", &self.editor.name_input_textarea);
        }
    }
    fn resource_label(&self, kind: Kind, r: &ResourceRef, width: usize) -> String {
        let marker = format!(
            "[{}{}{}{}]",
            if r.source == Source::Builtin {
                "B"
            } else {
                "C"
            },
            if kind == Kind::Theme && *r == self.session.store.base.active_theme {
                " A"
            } else {
                ""
            },
            if self.session.dirty(kind, r) {
                " *"
            } else {
                ""
            },
            if self.shared.contains(&(kind, r.clone())) {
                " Shared"
            } else {
                ""
            }
        );
        format!(
            "{} {}",
            marker,
            layout::clip(&r.name, width.saturating_sub(marker.len() + 1))
        )
    }
    fn resource_strip(&self, f: &mut Frame, area: Rect) {
        let refs = self.session.draft.list(self.kind);
        let index = refs.iter().position(|r| *r == self.current()).unwrap_or(0);
        let width = area.width as usize;
        let center = self.resource_label(self.kind, &self.current(), width.saturating_sub(16));
        let mut spans = vec![
            Span::raw(format!("‹ {}/{} ", index + 1, refs.len())),
            Span::styled(center, Style::default().fg(Color::Yellow)),
            Span::raw(" ›"),
        ];
        let used = Line::from(spans.clone()).width();
        if width > used + 12
            && let Some(next) = refs.get(index + 1)
        {
            spans.push(Span::styled(
                format!(
                    "  {}",
                    self.resource_label(self.kind, next, width - used - 2)
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }
        f.render_widget(Paragraph::new(Line::from(spans)), area);
    }
    fn theme_body(&self, f: &mut Frame, g: &Geometry) {
        let Ok(t) = self.session.draft.theme(&self.context) else {
            return;
        };
        let rows = [Kind::Components, Kind::Icons, Kind::Colors]
            .iter()
            .map(|k| format!("{}: {}", k.label(), t.reference(*k).label()))
            .collect::<Vec<_>>();
        draw_list(
            f,
            g.list,
            " Associations — Enter to assign ",
            &rows,
            self.association,
            self.editor.selected_panel == Panel::ComponentList,
        );
        if g.inspector.width > 0 {
            let kind = Kind::ALL[self.association + 1];
            let r = t.reference(kind);
            let deps = self.session.draft.dependents(kind, r);
            let body = format!(
                "{}\n{}\n\nUsed by {} themes.\n\nEnter: choose a named config\n{}: open this config's editor\n\nSaved edits to active references affect the next statusline.\n\n{}",
                kind.label(),
                r.label(),
                deps.len(),
                self.association + 2,
                t.description
            );
            f.render_widget(
                Paragraph::new(body)
                    .block(Block::bordered().title(" Composition "))
                    .wrap(Wrap { trim: false }),
                g.inspector,
            );
        }
    }
    fn component_list(&self, f: &mut Frame, area: Rect) {
        if area.width == 0 {
            return;
        }
        let rows = self
            .editor
            .theme
            .components
            .iter()
            .map(|c| {
                if c.id == ComponentId::Unknown {
                    "Defaults (fallback values)".into()
                } else if c.id == ComponentId::Separator {
                    if self.kind == Kind::Icons {
                        "Mode / Separators".into()
                    } else {
                        "Separator color".into()
                    }
                } else if self.kind == Kind::Components {
                    format!(
                        "[{}] {}",
                        if c.enabled { "x" } else { " " },
                        c.display_name()
                    )
                } else {
                    let inherited = match self.session.draft.get(self.kind, &self.current()) {
                        Ok(Resource::Icons(p)) => !p.components.contains_key(&c.id.key()),
                        Ok(Resource::Colors(p)) => !p.components.contains_key(&c.id.key()),
                        _ => false,
                    };
                    format!(
                        "{}{}",
                        c.display_name(),
                        if inherited { " (default)" } else { "" }
                    )
                }
            })
            .collect::<Vec<_>>();
        let count = self
            .session
            .draft
            .dependents(self.kind, &self.current())
            .len();
        draw_list(
            f,
            area,
            &format!(" {} · Used by {count} ", self.kind.label()),
            &rows,
            self.editor.selected_component,
            self.editor.selected_panel == Panel::ComponentList,
        );
    }
    fn draw_modal(&mut self, f: &mut Frame) {
        let Some(modal) = &self.modal else {
            return;
        };
        let popup = layout::popup(f.area());
        f.render_widget(Clear, popup);
        let outer = Block::bordered().border_style(Style::default().fg(Color::Blue));
        let inner = outer.inner(popup);
        f.render_widget(outer, popup);
        let rows = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(inner);
        self.modal_viewport = rows[1].height.max(1) as usize;
        let (title, footer) = match modal {
            Modal::Menu { index } => {
                let items = ACTIONS
                    .iter()
                    .map(|(_, l, k)| format!("{k}  {l}"))
                    .collect::<Vec<_>>();
                draw_list(f, rows[1], "", &items, *index, true);
                self.modal_viewport = rows[1].height.saturating_sub(2).max(1) as usize;
                ("Actions", "↑↓/Pg select  Enter accept  Esc cancel")
            }
            Modal::Choose {
                kind, query, index, ..
            } => {
                let parts = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(rows[1]);
                let mut input = query.clone();
                input.set_block(Block::bordered().title(" Search "));
                f.render_widget(&input, parts[0]);
                let refs = self.choices(*kind, query);
                let items = refs
                    .iter()
                    .map(|r| {
                        self.resource_label(*kind, r, parts[1].width.saturating_sub(4) as usize)
                    })
                    .collect::<Vec<_>>();
                draw_list(f, parts[1], kind.label(), &items, *index, true);
                self.modal_viewport = parts[1].height.saturating_sub(2).max(1) as usize;
                if let Ok(theme) = self.preview() {
                    let block = Block::bordered().title(" Preview selection ");
                    let line = Line::from(crate::core::render::render_spans(
                        &crate::core::render::demo_line(&theme),
                    ));
                    f.render_widget(
                        Paragraph::new(layout::clip_line(
                            line,
                            block.inner(parts[2]).width as usize,
                        ))
                        .block(block),
                        parts[2],
                    );
                }

                (
                    "Choose named resource",
                    "Type search  ↑↓/Pg preview  Enter choose  Esc cancel",
                )
            }
            Modal::Name {
                purpose,
                input,
                error,
                ..
            } => {
                let parts =
                    Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(rows[1]);
                let mut input = input.clone();
                input.set_block(Block::bordered().title(" Name "));
                f.render_widget(&input, parts[0]);
                let explanation = if matches!(purpose, Naming::Customize) {
                    "Creates a named config copy and retargets only the current theme. A built-in theme also gets a custom copy."
                } else if matches!(purpose, Naming::Rename) {
                    "Renaming updates all saved references atomically on Save. Unrelated theme drafts stay unsaved."
                } else if matches!(purpose, Naming::FullFork) {
                    "Creates a theme and three independent named configs. Save commits the complete fork."
                } else {
                    "A duplicated theme reuses its existing config references."
                };
                let details = if matches!(purpose, Naming::Rename) {
                    format!("\n\nAffected themes:\n{}", self.dependent_text())
                } else {
                    String::new()
                };
                f.render_widget(
                    Paragraph::new(format!(
                        "{}\n{explanation}{details}",
                        error.as_deref().unwrap_or_default()
                    ))
                    .wrap(Wrap { trim: false }),
                    parts[1],
                );
                ("Name custom resource", "Enter create draft  Esc cancel")
            }
            Modal::Confirm {
                action,
                body,
                scroll,
            } => {
                f.render_widget(
                    Paragraph::new(body.as_str())
                        .wrap(Wrap { trim: false })
                        .scroll((*scroll, 0)),
                    rows[1],
                );
                (
                    "Review action",
                    if matches!(action, Confirm::Exit) {
                        "s save all + exit  d discard + exit  Esc cancel"
                    } else {
                        "Enter/y confirm  Esc/n cancel  ↑↓/Pg scroll"
                    },
                )
            }
            Modal::Message {
                title,
                body,
                scroll,
            } => {
                f.render_widget(
                    Paragraph::new(body.as_str())
                        .wrap(Wrap { trim: false })
                        .scroll((*scroll, 0)),
                    rows[1],
                );
                (title.as_str(), "↑↓/Pg scroll  Enter/Esc close")
            }
        };
        f.render_widget(
            Paragraph::new(layout::clip(title, rows[0].width as usize))
                .style(Style::default().fg(Color::Yellow)),
            rows[0],
        );
        f.render_widget(
            Paragraph::new(footer).style(Style::default().fg(Color::Cyan)),
            rows[2],
        );
    }
}
fn scroll_key(scroll: &mut u16, code: KeyCode, page: usize) {
    match code {
        KeyCode::Up => *scroll = scroll.saturating_sub(1),
        KeyCode::Down => *scroll = scroll.saturating_add(1),
        KeyCode::PageUp => *scroll = scroll.saturating_sub(page as u16),
        KeyCode::PageDown => *scroll = scroll.saturating_add(page as u16),
        KeyCode::Home => *scroll = 0,
        _ => {}
    }
}
fn draw_list(
    f: &mut Frame,
    area: Rect,
    title: &str,
    rows: &[String],
    selected: usize,
    focused: bool,
) {
    if area.width == 0 {
        return;
    }
    let visible = area.height.saturating_sub(2) as usize;
    let offset = selected
        .saturating_sub(visible / 2)
        .min(rows.len().saturating_sub(visible));
    let items = rows
        .iter()
        .enumerate()
        .skip(offset)
        .take(visible)
        .map(|(i, s)| {
            ListItem::new(format!(
                "{}{}",
                if i == selected { "> " } else { "  " },
                layout::clip(s, area.width.saturating_sub(4) as usize)
            ))
            .style(Style::default().fg(if i == selected && focused {
                Color::Yellow
            } else {
                Color::Gray
            }))
        })
        .collect::<Vec<_>>();
    let block = Block::default()
        .borders(Borders::ALL)
        .title(layout::clip(title, area.width.saturating_sub(2) as usize))
        .title_bottom(format!(
            " {}/{} ↑↓ ",
            if rows.is_empty() { 0 } else { selected + 1 },
            rows.len()
        ))
        .border_style(Style::default().fg(if focused {
            Color::Blue
        } else {
            Color::DarkGray
        }));
    f.render_widget(List::new(items).block(block), area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};
    fn app() -> (tempfile::TempDir, App) {
        let dir = tempfile::tempdir().unwrap();
        let app = App::new(Store::open(dir.path()).unwrap());
        (dir, app)
    }
    fn key(app: &mut App, key: KeyCode) {
        app.handle_key(key, KeyModifiers::NONE);
    }
    fn screen(app: &mut App, w: u16, h: u16) -> String {
        let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
        terminal.draw(|f| app.ui(f)).unwrap();
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect::<String>()
    }
    #[test]
    fn every_size_and_section_preserves_focus_drafts_and_essential_controls() {
        let (_dir, mut a) = app();
        for (w, h) in [(60, 18), (80, 18), (80, 24), (80, 25), (80, 30), (120, 40)] {
            for section in ['1', '2', '3', '4'] {
                key(&mut a, KeyCode::Char(section));
                let text = screen(&mut a, w, h);
                assert!(text.contains("m actions"), "{w}x{h} section {section}");
                key(&mut a, KeyCode::Tab);
                let focus = *a.editor.selected_panel.current();
                screen(&mut a, 20, 6);
                screen(&mut a, w, h);
                assert_eq!(*a.editor.selected_panel.current(), focus);
                key(&mut a, KeyCode::Char('m'));
                assert!(screen(&mut a, w, h).contains("Esc cancel"));
                key(&mut a, KeyCode::Esc);
            }
        }
        assert!(!a.session.any_dirty());
        assert!(!a.session.store.path.exists());
    }
    #[test]
    fn customize_preserves_selected_field_and_pending_edit_with_navigation_drafts() {
        let (_dir, mut a) = app();
        key(&mut a, KeyCode::Char('2'));
        key(&mut a, KeyCode::Down);
        let id = a.editor.theme.components[a.editor.selected_component].id;
        assert_eq!(id, ComponentId::Model);
        key(&mut a, KeyCode::Enter);
        assert!(matches!(a.modal, Some(Modal::Name { .. })));
        key(&mut a, KeyCode::Enter);
        assert_eq!(
            a.editor.theme.components[a.editor.selected_component].id,
            id
        );
        assert!(!a.editor.theme.components[a.editor.selected_component].enabled);
        assert_eq!(a.current().source, Source::User);
        assert_eq!(a.context.source, Source::User);
        let r = a.current();
        let context = a.context.clone();
        key(&mut a, KeyCode::Char('4'));
        key(&mut a, KeyCode::Char('2'));
        assert!(a.session.dirty(Kind::Components, &r));
        assert!(!a.session.store.path.exists());
        key(&mut a, KeyCode::Char('s'));
        assert!(matches!(a.modal, Some(Modal::Confirm { .. })));
        key(&mut a, KeyCode::Enter);
        assert!(a.session.store.base.theme(&context).is_ok());
        assert!(!a.session.any_dirty());
    }
    #[test]
    fn association_picker_cancel_restores_preview_and_accept_only_changes_draft() {
        let (_dir, mut a) = app();
        let before = a.session.draft.clone();
        key(&mut a, KeyCode::Down);
        key(&mut a, KeyCode::Enter);
        key(&mut a, KeyCode::End);
        let temporary = a.preview_definition().unwrap();
        assert_ne!(
            temporary.icons,
            before.theme(&before.active_theme).unwrap().icons
        );
        key(&mut a, KeyCode::Esc);
        assert_eq!(a.session.draft, before);
        key(&mut a, KeyCode::Enter);
        key(&mut a, KeyCode::End);
        key(&mut a, KeyCode::Enter);
        assert_eq!(
            a.session.draft.theme(&a.context).unwrap().icons,
            temporary.icons
        );
        assert_eq!(a.context.source, Source::User);
        assert!(!a.session.store.path.exists());
    }
    #[test]
    fn icon_picker_scroll_uses_real_viewport_after_resize() {
        let (_dir, mut a) = app();
        key(&mut a, KeyCode::Char('3'));
        key(&mut a, KeyCode::Tab);
        key(&mut a, KeyCode::Down);
        key(&mut a, KeyCode::Down);
        key(&mut a, KeyCode::Down); // separator plain glyph
        key(&mut a, KeyCode::Enter);
        key(&mut a, KeyCode::Enter);
        assert_eq!(a.editor.modal, Some(EditorModal::Icon));
        for (w, h) in [(120, 40), (60, 18), (80, 24)] {
            screen(&mut a, w, h);
            key(&mut a, KeyCode::PageDown);
            screen(&mut a, w, h);
            let sections = a.editor.icon_catalog.sections(
                *a.editor.icon_picker.tab.current(),
                a.editor.icon_picker.search_query(),
            );
            let selected = widgets::icon_picker::selectable_to_flat(
                &sections,
                a.editor.icon_picker.selected_index,
            )
            .unwrap();
            assert!(selected >= a.editor.icon_picker.scroll_offset);
            assert!(selected < a.editor.icon_picker.scroll_offset + a.editor.icon_viewport);
        }
        key(&mut a, KeyCode::Esc);
        assert_eq!(a.editor.modal, None);
    }
    #[test]
    fn failed_save_activate_exit_retains_drafts_and_active_selection() {
        let (_dir, mut a) = app();
        key(&mut a, KeyCode::Char('d'));
        key(&mut a, KeyCode::Enter);
        std::fs::write(&a.session.store.path, "changed externally").unwrap();
        key(&mut a, KeyCode::Char('x'));
        key(&mut a, KeyCode::Enter);
        assert!(!a.should_quit);
        assert!(a.session.any_dirty());
        assert_eq!(
            a.session.store.base.active_theme,
            ResourceRef::builtin("Default")
        );
        assert!(matches!(a.modal, Some(Modal::Message { .. })));
    }
}
