use crate::config::manager;
use crate::config::theme::UserTheme;
use crate::config::types::{
    AnsiColor, ComponentId, DEFAULT_GIT_AUTOHIDE_BRANCH, DEFAULT_HOSTNAME_RSTRIP,
    DEFAULT_PR_OSC_HYPERLINKS, DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL,
    DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH, GIT_OPTION_AUTOHIDE_BRANCH, MODEL_OPTION_REPLACE,
    MODEL_OPTION_SEARCH, MODEL_OPTION_SHOW_EFFORT, PR_OPTION_OSC_HYPERLINKS,
    PR_OPTION_SHOW_REVIEW_STATE, PR_OPTION_SHOW_URL, StyleMode, USAGE_OPTION_VALUE, UsageValue,
    WORKTREE_OPTION_OUTSIDE_WORKTREES, WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH, WorktreeOutside,
};
use crate::core::ring_cursor::RingCursor;
use crate::data::icon_catalog::{IconCatalogData, IconPickerTab};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{Frame, style::Style};
use ratatui_textarea::{CursorMove, TextArea};
use std::path::PathBuf;

use super::widgets::{
    component_list::ComponentListWidget,
    editor::{EditorWidget, FieldSelection},
    help_bar::HelpBarWidget,
    preview::PreviewWidget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    ComponentList,
    Editor,
}

pub struct App {
    // Theme state
    pub theme: UserTheme,
    pub theme_name: String,
    pub theme_path: PathBuf,
    pub is_dirty: bool,

    // Theme bar state
    pub theme_list: Vec<(String, PathBuf)>,
    pub theme_list_index: usize,
    pub active_theme_name: Option<String>,

    // UI state
    pub selected_component: usize,
    pub selected_panel: RingCursor<Panel>,
    pub selected_field: FieldSelection,
    pub should_quit: bool,
    pub status_message: Option<String>,

    // Popup state
    pub file_menu_open: bool,
    pub file_menu_selection: RingCursor<FileMenuAction>,
    pub import_colors_open: bool,
    pub import_colors_selection: usize,
    pub import_icons_open: bool,
    pub import_icons_selection: usize,
    pub open_menu_open: bool,
    pub open_menu_selection: usize,
    pub open_menu_themes: Vec<(String, PathBuf)>,
    pub name_input_open: bool,
    pub name_input_textarea: TextArea<'static>,
    pub name_input_purpose: NameInputPurpose,
    pub pending_open_theme: Option<(String, PathBuf)>,
    pub confirm_dialog_open: bool,
    pub confirm_dialog_message: String,
    pub confirm_dialog_action: ConfirmAction,
    pub color_picker_open: bool,
    pub color_picker: ColorPickerState,
    pub icon_picker_open: bool,
    pub icon_picker: IconPickerState,
    pub icon_catalog: IconCatalogData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameInputPurpose {
    SaveAs,
    Rename,
    HostnameRstrip,
    ModelSearch,
    ModelReplace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmAction {
    DeleteTheme,
    ExitWithoutSaving,
    DiscardAndOpen,
    ReinstallDefaults,
}

impl ConfirmAction {
    pub fn hints(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            ConfirmAction::ReinstallDefaults => &[("O", "OK"), ("C", "Cancel")],
            _ => &[("Y", "Yes"), ("N", "No")],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMenuAction {
    SaveActivateExit,
    Activate,
    Save,
    SaveAs,
    Open,
    Rename,
    Delete,
    ReinstallDefaults,
    Exit,
}

#[derive(Debug, Clone)]
pub struct ColorPickerState {
    pub mode: RingCursor<ColorPickerMode>,
    pub c16_selection: u8,
    pub c256_selection: u8,
    pub rgb_textareas: [TextArea<'static>; 3],
    pub rgb_focus: usize, // 0=R, 1=G, 2=B
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPickerMode {
    Color16,
    Color256,
    Rgb,
}

impl ColorPickerState {
    pub fn r_val(&self) -> u8 {
        self.rgb_textareas[0]
            .lines()
            .first()
            .and_then(|s| s.parse().ok())
            .unwrap_or(128)
    }

    pub fn g_val(&self) -> u8 {
        self.rgb_textareas[1]
            .lines()
            .first()
            .and_then(|s| s.parse().ok())
            .unwrap_or(128)
    }

    pub fn b_val(&self) -> u8 {
        self.rgb_textareas[2]
            .lines()
            .first()
            .and_then(|s| s.parse().ok())
            .unwrap_or(128)
    }

    pub fn r_str(&self) -> &str {
        self.rgb_textareas[0]
            .lines()
            .first()
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn g_str(&self) -> &str {
        self.rgb_textareas[1]
            .lines()
            .first()
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn b_str(&self) -> &str {
        self.rgb_textareas[2]
            .lines()
            .first()
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        for (i, val) in [r, g, b].iter().enumerate() {
            let mut ta = TextArea::new(vec![val.to_string()]);
            ta.move_cursor(CursorMove::End);
            ta.set_cursor_line_style(Style::default());
            self.rgb_textareas[i] = ta;
        }
    }
}

impl Default for ColorPickerState {
    fn default() -> Self {
        let mut r = TextArea::new(vec!["128".into()]);
        r.move_cursor(CursorMove::End);
        r.set_cursor_line_style(Style::default());
        let mut g = TextArea::new(vec!["128".into()]);
        g.move_cursor(CursorMove::End);
        g.set_cursor_line_style(Style::default());
        let mut b = TextArea::new(vec!["128".into()]);
        b.move_cursor(CursorMove::End);
        b.set_cursor_line_style(Style::default());

        Self {
            mode: RingCursor::new(vec![
                ColorPickerMode::Color16,
                ColorPickerMode::Color256,
                ColorPickerMode::Rgb,
            ]),
            c16_selection: 0,
            c256_selection: 0,
            rgb_textareas: [r, g, b],
            rgb_focus: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPickerPurpose {
    PlainIcon,
    NerdFontIcon,
    ThinkingIcon,
    FlashIcon,
    ProIcon,
    UltraIcon,
    FlashLiteIcon,
    OpusIcon,
    SonnetIcon,
    HaikuIcon,
    FableIcon,
    MythosIcon,
}

#[derive(Debug, Clone)]
pub struct IconPickerState {
    pub tab: RingCursor<IconPickerTab>,
    pub purpose: IconPickerPurpose,
    pub search_textarea: TextArea<'static>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub custom_textarea: TextArea<'static>,
}

impl IconPickerState {
    pub fn search_query(&self) -> &str {
        self.search_textarea
            .lines()
            .first()
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn custom_buffer(&self) -> &str {
        self.custom_textarea
            .lines()
            .first()
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn set_search_query(&mut self, query: &str) {
        let mut ta = TextArea::new(vec![query.to_string()]);
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        self.search_textarea = ta;
    }

    pub fn set_custom_buffer(&mut self, text: &str) {
        let mut ta = TextArea::new(vec![text.to_string()]);
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        self.custom_textarea = ta;
    }
}

impl Default for IconPickerState {
    fn default() -> Self {
        let mut search_ta = TextArea::default();
        search_ta.set_cursor_line_style(Style::default());
        let mut custom_ta = TextArea::default();
        custom_ta.set_cursor_line_style(Style::default());

        Self {
            tab: RingCursor::new(vec![
                IconPickerTab::Emoji,
                IconPickerTab::NerdFont,
                IconPickerTab::Unicode,
                IconPickerTab::Custom,
            ]),
            purpose: IconPickerPurpose::PlainIcon,
            search_textarea: search_ta,
            selected_index: 0,
            scroll_offset: 0,
            custom_textarea: custom_ta,
        }
    }
}

impl FileMenuAction {
    fn all() -> Vec<Self> {
        vec![
            Self::SaveActivateExit,
            Self::Activate,
            Self::Save,
            Self::SaveAs,
            Self::Open,
            Self::Rename,
            Self::Delete,
            Self::ReinstallDefaults,
            Self::Exit,
        ]
    }
}

pub const FILE_MENU_ITEMS: &[(&str, char)] = &[
    ("Save, activate, and exit", 's'),
    ("Activate", 'a'),
    ("Save", 'v'),
    ("Save as...", 'e'),
    ("Open...", 'o'),
    ("Rename...", 'r'),
    ("Delete", 'd'),
    ("Reinstall default themes", 'i'),
    ("Exit", 'x'),
];

impl App {
    pub fn new(name: String, path: PathBuf, theme: UserTheme) -> Self {
        let theme_list = manager::list_themes().unwrap_or_default();
        let theme_list_index = theme_list.iter().position(|(n, _)| n == &name).unwrap_or(0);
        let active_name = if theme.active {
            Some(name.clone())
        } else {
            // Find which theme is actually active
            theme_list.iter().find_map(|(n, p)| {
                manager::load_theme(p)
                    .ok()
                    .and_then(|t| if t.active { Some(n.clone()) } else { None })
            })
        };

        let mut name_input_ta = TextArea::default();
        name_input_ta.set_cursor_line_style(Style::default());

        Self {
            theme,
            theme_name: name,
            theme_path: path,
            is_dirty: false,
            theme_list,
            theme_list_index,
            active_theme_name: active_name,
            selected_component: 0,
            selected_panel: RingCursor::new(vec![Panel::ComponentList, Panel::Editor]),
            selected_field: FieldSelection::Enabled,
            should_quit: false,
            status_message: None,
            file_menu_open: false,
            file_menu_selection: RingCursor::new(FileMenuAction::all()),
            import_colors_open: false,
            import_colors_selection: 0,
            import_icons_open: false,
            import_icons_selection: 0,
            open_menu_open: false,
            open_menu_selection: 0,
            open_menu_themes: Vec::new(),
            name_input_open: false,
            name_input_textarea: name_input_ta,
            name_input_purpose: NameInputPurpose::SaveAs,
            pending_open_theme: None,
            confirm_dialog_open: false,
            confirm_dialog_message: String::new(),
            confirm_dialog_action: ConfirmAction::ExitWithoutSaving,
            color_picker_open: false,
            color_picker: ColorPickerState::default(),
            icon_picker_open: false,
            icon_picker: IconPickerState::default(),
            icon_catalog: IconCatalogData::load(),
        }
    }

    pub fn name_input_buffer(&self) -> String {
        self.name_input_textarea
            .lines()
            .first()
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_name_input_buffer(&mut self, s: &str) {
        let mut ta = TextArea::new(vec![s.to_string()]);
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        self.name_input_textarea = ta;
    }

    pub fn open_name_input(&mut self, purpose: NameInputPurpose, initial: &str) {
        let mut ta = TextArea::new(vec![initial.to_string()]);
        ta.move_cursor(CursorMove::End);
        ta.set_cursor_line_style(Style::default());
        self.name_input_textarea = ta;
        self.name_input_purpose = purpose;
        self.name_input_open = true;
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn refresh_theme_list(&mut self) {
        self.theme_list = manager::list_themes().unwrap_or_default();
        self.theme_list_index = self
            .theme_list
            .iter()
            .position(|(n, _)| n == &self.theme_name)
            .unwrap_or(0);
    }

    /// Number of reorderable components (everything except Separator)
    fn reorderable_count(&self) -> usize {
        self.theme
            .components
            .iter()
            .filter(|c| c.id != ComponentId::Separator)
            .count()
    }

    /// Total component count for navigation
    fn component_count(&self) -> usize {
        self.theme.components.len()
    }

    // --- Event handling ---

    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Ctrl+C: immediate quit from anywhere, no prompt
        if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }

        let is_cancel = code == KeyCode::Esc;

        // Handle popups first (highest priority)
        if self.confirm_dialog_open {
            self.handle_confirm_dialog(code, is_cancel);
            return;
        }
        if self.name_input_open {
            self.handle_name_input_with_modifiers(code, modifiers, is_cancel);
            return;
        }
        if self.color_picker_open {
            self.handle_color_picker(code, modifiers, is_cancel);
            return;
        }
        if self.icon_picker_open {
            self.handle_icon_picker_with_modifiers(code, modifiers, is_cancel);
            return;
        }
        if self.import_colors_open {
            self.handle_import_colors(code, is_cancel);
            return;
        }
        if self.import_icons_open {
            self.handle_import_icons(code, is_cancel);
            return;
        }
        if self.open_menu_open {
            self.handle_open_menu(code, is_cancel);
            return;
        }
        if self.file_menu_open {
            self.handle_file_menu(code, is_cancel);
            return;
        }

        // Main app keys
        match code {
            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.file_menu_open = true;
                self.file_menu_selection
                    .set(&FileMenuAction::SaveActivateExit);
            }
            KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => {
                if self.is_dirty {
                    self.confirm_dialog_open = true;
                    self.confirm_dialog_message = "Exit without saving changes?".into();
                    self.confirm_dialog_action = ConfirmAction::ExitWithoutSaving;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                if self.is_dirty {
                    self.confirm_dialog_open = true;
                    self.confirm_dialog_message = "Exit without saving changes?".into();
                    self.confirm_dialog_action = ConfirmAction::ExitWithoutSaving;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Esc => {
                self.file_menu_open = true;
                self.file_menu_selection
                    .set(&FileMenuAction::SaveActivateExit);
            }
            KeyCode::Tab => {
                self.selected_panel.move_next();
            }
            KeyCode::BackTab => {
                self.selected_panel.move_prev();
            }
            KeyCode::Up => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.move_component_up();
                } else {
                    self.move_selection(-1);
                }
            }
            KeyCode::Down => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.move_component_down();
                } else {
                    self.move_selection(1);
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => self.toggle_current(),
            KeyCode::Left => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.switch_theme(-1);
                } else {
                    self.selected_panel.move_next();
                }
            }
            KeyCode::Right => {
                if modifiers.contains(KeyModifiers::SHIFT) {
                    self.switch_theme(1);
                } else {
                    self.selected_panel.move_next();
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => self.switch_theme(-1),
            KeyCode::Char('d') | KeyCode::Char('D') => self.switch_theme(1),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.import_colors_open = true;
                self.import_colors_selection = 0;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.import_icons_open = true;
                self.import_icons_selection = 0;
            }
            _ => {}
        }
    }

    fn move_selection(&mut self, delta: i32) {
        match *self.selected_panel.current() {
            Panel::ComponentList => {
                let count = self.component_count();
                if count > 0 {
                    self.selected_component =
                        (self.selected_component as i32 + delta).rem_euclid(count as i32) as usize;
                }
                // Clamp field selection to valid fields for the new component
                if let Some(comp) = self.theme.components.get(self.selected_component) {
                    let fields = FieldSelection::fields_for(comp);
                    if !fields.contains(&self.selected_field) {
                        self.selected_field = FieldSelection::Enabled;
                    }
                }
            }
            Panel::Editor => {
                if let Some(comp) = self.theme.components.get(self.selected_component) {
                    let fields = FieldSelection::fields_for(comp);
                    let current = fields
                        .iter()
                        .position(|f| *f == self.selected_field)
                        .unwrap_or(0) as i32;
                    let new_idx = (current + delta).clamp(0, fields.len() as i32 - 1) as usize;
                    self.selected_field = fields[new_idx];
                }
            }
        }
    }

    fn toggle_current(&mut self) {
        match *self.selected_panel.current() {
            Panel::ComponentList => {
                if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                    comp.enabled = !comp.enabled;
                    self.status_message = Some(format!(
                        "{} {}",
                        comp.id.display_name(),
                        if comp.enabled { "enabled" } else { "disabled" }
                    ));
                    self.mark_dirty();
                }
            }
            Panel::Editor => {
                if self.selected_component < self.theme.components.len() {
                    match self.selected_field {
                        FieldSelection::Enabled => {
                            let comp = &mut self.theme.components[self.selected_component];
                            comp.enabled = !comp.enabled;
                            self.status_message = Some(format!(
                                "{} {}",
                                comp.id.display_name(),
                                if comp.enabled { "enabled" } else { "disabled" }
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::StyleMode => {
                            self.theme.style.mode = match self.theme.style.mode {
                                StyleMode::Plain => StyleMode::NerdFont,
                                StyleMode::NerdFont => StyleMode::Powerline,
                                StyleMode::Powerline => StyleMode::PlainPowerline,
                                StyleMode::PlainPowerline => StyleMode::Plain,
                            };
                            self.status_message =
                                Some(format!("Style: {}", self.theme.style.mode.display_name()));
                            self.mark_dirty();
                        }
                        FieldSelection::PlainIcon => {
                            self.open_icon_picker(IconPickerPurpose::PlainIcon);
                        }
                        FieldSelection::NerdFontIcon => {
                            self.open_icon_picker(IconPickerPurpose::NerdFontIcon);
                        }
                        FieldSelection::HostnameRstrip => {
                            let value = self.theme.components[self.selected_component]
                                .options
                                .get("rstrip")
                                .and_then(|value| value.as_str())
                                .unwrap_or(DEFAULT_HOSTNAME_RSTRIP)
                                .to_owned();
                            self.open_name_input(NameInputPurpose::HostnameRstrip, &value);
                        }
                        FieldSelection::WorktreeOutside => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let mode = WorktreeOutside::from_options(&comp.options).toggled();
                            comp.options.insert(
                                WORKTREE_OPTION_OUTSIDE_WORKTREES.into(),
                                mode.as_str().into(),
                            );
                            self.status_message =
                                Some(format!("Outside worktrees: {}", mode.display_name()));
                            self.mark_dirty();
                        }
                        FieldSelection::WorktreeOriginalBranch => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let enabled = comp
                                .options
                                .get(WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH)
                                .and_then(|value| value.as_bool())
                                .unwrap_or(DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH);
                            comp.options.insert(
                                WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH.into(),
                                serde_json::Value::Bool(!enabled),
                            );
                            self.status_message = Some(format!(
                                "Worktree original branch {}",
                                if enabled { "hidden" } else { "shown" }
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::GitAutohideBranch => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let enabled = comp
                                .options
                                .get(GIT_OPTION_AUTOHIDE_BRANCH)
                                .and_then(|value| value.as_bool())
                                .unwrap_or(DEFAULT_GIT_AUTOHIDE_BRANCH);
                            comp.options.insert(
                                GIT_OPTION_AUTOHIDE_BRANCH.into(),
                                serde_json::Value::Bool(!enabled),
                            );
                            self.status_message = Some(format!(
                                "Git branch autohide {}",
                                if enabled { "disabled" } else { "enabled" }
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::PrReviewState => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let enabled = comp
                                .options
                                .get(PR_OPTION_SHOW_REVIEW_STATE)
                                .and_then(|value| value.as_bool())
                                .unwrap_or(DEFAULT_PR_SHOW_REVIEW_STATE);
                            comp.options.insert(
                                PR_OPTION_SHOW_REVIEW_STATE.into(),
                                serde_json::Value::Bool(!enabled),
                            );
                            self.status_message = Some(format!(
                                "PR review state {}",
                                if enabled { "hidden" } else { "shown" }
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::PrUrl => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let enabled = comp
                                .options
                                .get(PR_OPTION_SHOW_URL)
                                .and_then(|value| value.as_bool())
                                .unwrap_or(DEFAULT_PR_SHOW_URL);
                            comp.options.insert(
                                PR_OPTION_SHOW_URL.into(),
                                serde_json::Value::Bool(!enabled),
                            );
                            self.status_message = Some(format!(
                                "PR URL {}",
                                if enabled { "hidden" } else { "shown" }
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::PrOscHyperlinks => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let enabled = comp
                                .options
                                .get(PR_OPTION_OSC_HYPERLINKS)
                                .and_then(|value| value.as_bool())
                                .unwrap_or(DEFAULT_PR_OSC_HYPERLINKS);
                            comp.options.insert(
                                PR_OPTION_OSC_HYPERLINKS.into(),
                                serde_json::Value::Bool(!enabled),
                            );
                            self.status_message = Some(format!(
                                "PR OSC hyperlinks {}",
                                if enabled { "disabled" } else { "enabled" }
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::UsageValue => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let value = UsageValue::from_options(&comp.options).toggled();
                            comp.options
                                .insert(USAGE_OPTION_VALUE.into(), value.as_str().into());
                            self.status_message = Some(format!(
                                "{} value: {}",
                                comp.id.display_name(),
                                value.display_name()
                            ));
                            self.mark_dirty();
                        }
                        FieldSelection::PerModelIcons => {
                            let comp = &mut self.theme.components[self.selected_component];
                            if let Some(pm) = &mut comp.icon.per_model {
                                pm.enabled = !pm.enabled;
                            } else {
                                comp.icon.per_model =
                                    Some(crate::config::types::PerModelIcons::default());
                            }
                            let pm_enabled =
                                comp.icon.per_model.as_ref().is_some_and(|pm| pm.enabled);
                            self.status_message = Some(format!(
                                "Per-model icons {}",
                                if pm_enabled { "enabled" } else { "disabled" }
                            ));
                            // Clamp selected_field to stay in valid range
                            let fields = FieldSelection::fields_for(
                                &self.theme.components[self.selected_component],
                            );
                            if !fields.contains(&self.selected_field) {
                                self.selected_field = FieldSelection::PerModelIcons;
                            }
                            self.mark_dirty();
                        }
                        FieldSelection::EffortLevel => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let next =
                                crate::config::types::ModelEffort::from_options(&comp.options)
                                    .toggled();
                            comp.options.insert(
                                MODEL_OPTION_SHOW_EFFORT.into(),
                                serde_json::Value::String(next.as_str().into()),
                            );
                            self.status_message =
                                Some(format!("Effort level {}", next.display_name()));
                            self.mark_dirty();
                        }
                        FieldSelection::ThinkingIcon => {
                            self.open_icon_picker(IconPickerPurpose::ThinkingIcon);
                        }
                        FieldSelection::ModelSearch => {
                            let value = self.theme.components[self.selected_component]
                                .options
                                .get(MODEL_OPTION_SEARCH)
                                .and_then(|value| value.as_str())
                                .unwrap_or_default()
                                .to_owned();
                            self.open_name_input(NameInputPurpose::ModelSearch, &value);
                        }
                        FieldSelection::ModelReplace => {
                            let value = self.theme.components[self.selected_component]
                                .options
                                .get(MODEL_OPTION_REPLACE)
                                .and_then(|value| value.as_str())
                                .unwrap_or_default()
                                .to_owned();
                            self.open_name_input(NameInputPurpose::ModelReplace, &value);
                        }
                        FieldSelection::FlashIcon => {
                            self.open_icon_picker(IconPickerPurpose::FlashIcon);
                        }
                        FieldSelection::ProIcon => {
                            self.open_icon_picker(IconPickerPurpose::ProIcon);
                        }
                        FieldSelection::UltraIcon => {
                            self.open_icon_picker(IconPickerPurpose::UltraIcon);
                        }
                        FieldSelection::FlashLiteIcon => {
                            self.open_icon_picker(IconPickerPurpose::FlashLiteIcon);
                        }
                        FieldSelection::OpusIcon => {
                            self.open_icon_picker(IconPickerPurpose::OpusIcon);
                        }
                        FieldSelection::SonnetIcon => {
                            self.open_icon_picker(IconPickerPurpose::SonnetIcon);
                        }
                        FieldSelection::FableIcon => {
                            self.open_icon_picker(IconPickerPurpose::FableIcon);
                        }
                        FieldSelection::MythosIcon => {
                            self.open_icon_picker(IconPickerPurpose::MythosIcon);
                        }
                        FieldSelection::HaikuIcon => {
                            self.open_icon_picker(IconPickerPurpose::HaikuIcon);
                        }
                        FieldSelection::IconColor
                        | FieldSelection::TextColor
                        | FieldSelection::BackgroundColor => {
                            self.open_color_picker();
                        }
                        FieldSelection::Bold => {
                            let comp = &mut self.theme.components[self.selected_component];
                            comp.styles.text_bold = !comp.styles.text_bold;
                            self.status_message = Some(format!(
                                "Bold {}",
                                if comp.styles.text_bold {
                                    "enabled"
                                } else {
                                    "disabled"
                                }
                            ));
                            self.mark_dirty();
                        }
                    }
                }
            }
        }
    }

    fn move_component_up(&mut self) {
        if self.selected_panel == Panel::ComponentList && self.selected_component > 0 {
            let reorderable = self.reorderable_count();
            if self.selected_component < reorderable {
                self.theme
                    .components
                    .swap(self.selected_component, self.selected_component - 1);
                self.selected_component -= 1;
                self.mark_dirty();
            }
        }
    }

    fn move_component_down(&mut self) {
        if self.selected_panel == Panel::ComponentList {
            let reorderable = self.reorderable_count();
            if self.selected_component + 1 < reorderable {
                self.theme
                    .components
                    .swap(self.selected_component, self.selected_component + 1);
                self.selected_component += 1;
                self.mark_dirty();
            }
        }
    }

    // --- File menu ---

    fn handle_file_menu(&mut self, code: KeyCode, is_cancel: bool) {
        if is_cancel {
            self.file_menu_open = false;
            return;
        }
        match code {
            KeyCode::Up => {
                self.file_menu_selection.move_prev();
            }
            KeyCode::Down => {
                self.file_menu_selection.move_next();
            }
            KeyCode::Char(c) => {
                if let Some(idx) = FILE_MENU_ITEMS
                    .iter()
                    .position(|(_, mnemonic)| mnemonic.eq_ignore_ascii_case(&c))
                {
                    let action = FileMenuAction::all()[idx];
                    self.file_menu_selection.set(&action);
                }
            }
            KeyCode::Enter => {
                self.file_menu_open = false;
                match *self.file_menu_selection.current() {
                    FileMenuAction::SaveActivateExit => self.action_save_activate_exit(),
                    FileMenuAction::Activate => self.action_activate(),
                    FileMenuAction::Save => self.action_save(),
                    FileMenuAction::SaveAs => self.action_save_as(),
                    FileMenuAction::Open => self.action_open(),
                    FileMenuAction::Rename => self.action_rename(),
                    FileMenuAction::Delete => self.action_delete(),
                    FileMenuAction::ReinstallDefaults => self.action_reinstall_defaults(),
                    FileMenuAction::Exit => self.action_exit(),
                }
            }
            _ => {}
        }
    }

    fn action_save_activate_exit(&mut self) {
        self.action_save();
        match manager::activate_theme(&self.theme_name) {
            Ok(()) => {
                self.active_theme_name = Some(self.theme_name.clone());
                self.should_quit = true;
            }
            Err(e) => self.status_message = Some(format!("Activate error: {}", e)),
        }
    }

    fn action_activate(&mut self) {
        if self.is_dirty {
            self.action_save();
        }
        match manager::activate_theme(&self.theme_name) {
            Ok(()) => {
                self.active_theme_name = Some(self.theme_name.clone());
                self.status_message = Some(format!("{} activated", self.theme_name));
            }
            Err(e) => self.status_message = Some(format!("Activate error: {}", e)),
        }
    }

    fn action_save(&mut self) {
        self.theme.active = if self.theme_path.exists() {
            {
                manager::load_theme(&self.theme_path)
                    .map(|t| t.active)
                    .unwrap_or(false)
            }
        } else {
            false
        };

        match manager::save_theme(&self.theme_path, &self.theme) {
            Ok(()) => {
                self.is_dirty = false;
                self.status_message = Some(format!("{} saved", self.theme_name));
            }
            Err(e) => self.status_message = Some(format!("Save error: {}", e)),
        }
    }

    fn action_save_as(&mut self) {
        self.open_name_input(NameInputPurpose::SaveAs, "");
    }

    fn action_open(&mut self) {
        match manager::list_themes() {
            Ok(themes) => {
                if themes.is_empty() {
                    self.status_message = Some("No themes found".into());
                    return;
                }
                self.open_menu_themes = themes;
                self.open_menu_selection = 0;
                self.open_menu_open = true;
            }
            Err(e) => self.status_message = Some(format!("Error listing themes: {}", e)),
        }
    }

    fn action_rename(&mut self) {
        let name = self.theme_name.clone();
        self.open_name_input(NameInputPurpose::Rename, &name);
    }

    fn action_delete(&mut self) {
        self.confirm_dialog_open = true;
        self.confirm_dialog_message = format!("Delete {}?", self.theme_name);
        self.confirm_dialog_action = ConfirmAction::DeleteTheme;
    }

    fn action_reinstall_defaults(&mut self) {
        self.confirm_dialog_open = true;
        self.confirm_dialog_message =
            "Customized defaults will be overwritten.\nAre you sure?".into();
        self.confirm_dialog_action = ConfirmAction::ReinstallDefaults;
    }

    fn action_exit(&mut self) {
        if self.is_dirty {
            self.confirm_dialog_open = true;
            self.confirm_dialog_message = "Exit without saving changes?".into();
            self.confirm_dialog_action = ConfirmAction::ExitWithoutSaving;
        } else {
            self.should_quit = true;
        }
    }

    // --- Name input ---

    pub fn handle_name_input(&mut self, code: KeyCode, is_cancel: bool) {
        self.handle_name_input_with_modifiers(code, KeyModifiers::NONE, is_cancel);
    }

    pub fn handle_name_input_with_modifiers(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
        is_cancel: bool,
    ) {
        if is_cancel {
            self.name_input_open = false;
            return;
        }
        match code {
            KeyCode::Enter => {
                self.name_input_open = false;
                let text = self
                    .name_input_textarea
                    .lines()
                    .first()
                    .map(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                match self.name_input_purpose {
                    NameInputPurpose::SaveAs | NameInputPurpose::Rename => {
                        let name = text.trim().to_string();
                        if name.is_empty() || !manager::is_valid_theme_name(&name) {
                            self.status_message = Some("Invalid theme name".into());
                            return;
                        }
                        if self.name_input_purpose == NameInputPurpose::SaveAs {
                            self.finish_save_as(&name);
                        } else {
                            self.finish_rename(&name);
                        }
                    }
                    NameInputPurpose::HostnameRstrip => {
                        let value = text.trim().to_owned();
                        if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                            comp.options
                                .insert("rstrip".into(), serde_json::Value::String(value));
                            self.status_message = Some("Hostname RStrip updated".into());
                            self.mark_dirty();
                        }
                    }
                    NameInputPurpose::ModelSearch => {
                        let value = text;
                        if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                            comp.options.insert(
                                MODEL_OPTION_SEARCH.into(),
                                serde_json::Value::String(value),
                            );
                            self.status_message = Some("Model Search pattern updated".into());
                            self.mark_dirty();
                        }
                    }
                    NameInputPurpose::ModelReplace => {
                        let value = text;
                        if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                            comp.options.insert(
                                MODEL_OPTION_REPLACE.into(),
                                serde_json::Value::String(value),
                            );
                            self.status_message = Some("Model Replacement updated".into());
                            self.mark_dirty();
                        }
                    }
                }
            }
            _ => {
                let key_event = crossterm::event::KeyEvent::new(code, modifiers);
                self.name_input_textarea.input(key_event);
            }
        }
    }

    fn finish_save_as(&mut self, name: &str) {
        let new_path = manager::theme_path(name);
        if new_path.exists() {
            self.status_message = Some(format!("{} already exists", name));
            return;
        }
        let mut new_theme = self.theme.clone();
        new_theme.active = false;
        match manager::save_theme(&new_path, &new_theme) {
            Ok(()) => {
                self.theme_name = name.to_string();
                self.theme_path = new_path;
                self.theme = new_theme;
                self.is_dirty = false;
                self.refresh_theme_list();
                self.status_message = Some(format!("Saved as {}", name));
            }
            Err(e) => self.status_message = Some(format!("Save error: {}", e)),
        }
    }

    fn finish_rename(&mut self, name: &str) {
        match manager::rename_theme(&self.theme_path, name) {
            Ok(new_path) => {
                self.theme_name = name.to_string();
                self.theme_path = new_path;
                self.refresh_theme_list();
                self.status_message = Some(format!("Renamed to {}", name));
            }
            Err(e) => self.status_message = Some(format!("Rename error: {}", e)),
        }
    }

    // --- Confirm dialog ---

    fn handle_confirm_dialog(&mut self, code: KeyCode, is_cancel: bool) {
        if is_cancel || matches!(code, KeyCode::Char('n' | 'N' | 'c' | 'C')) {
            self.confirm_dialog_open = false;
            return;
        }
        if matches!(code, KeyCode::Char('y' | 'Y' | 'o' | 'O') | KeyCode::Enter) {
            self.confirm_dialog_open = false;
            match self.confirm_dialog_action {
                ConfirmAction::DeleteTheme => {
                    match manager::delete_theme(&self.theme_path) {
                        Ok(()) => {
                            // Load another theme
                            match manager::load_active_theme() {
                                Ok((name, path, theme)) => {
                                    self.theme_name = name;
                                    self.theme_path = path;
                                    self.theme = theme;
                                    self.is_dirty = false;
                                    self.selected_component = 0;
                                    self.status_message = Some("Theme deleted".into());
                                }
                                Err(_) => {
                                    // Bootstrap will create Default
                                    let _ = manager::bootstrap();
                                    if let Ok((name, path, theme)) = manager::load_active_theme() {
                                        self.theme_name = name;
                                        self.theme_path = path;
                                        self.theme = theme;
                                        self.is_dirty = false;
                                    }
                                    self.status_message =
                                        Some("Theme deleted, loaded Default".into());
                                }
                            }
                        }
                        Err(e) => self.status_message = Some(format!("Delete error: {}", e)),
                    }
                }
                ConfirmAction::ExitWithoutSaving => {
                    self.should_quit = true;
                }
                ConfirmAction::DiscardAndOpen => {
                    if let Some((name, path)) = self.pending_open_theme.take() {
                        self.do_open_theme(&name, &path);
                    }
                }
                ConfirmAction::ReinstallDefaults => {
                    let dir = manager::themes_dir();
                    match manager::write_default_themes(&dir, true) {
                        Ok(n) => {
                            // Reload current theme if its file was overwritten
                            if let Ok(reloaded) = manager::load_theme(&self.theme_path) {
                                self.theme = reloaded;
                                self.is_dirty = false;
                            }
                            self.refresh_theme_list();
                            self.status_message = Some(format!("Reinstalled {} default themes", n));
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Reinstall error: {}", e));
                        }
                    }
                }
            }
        }
    }

    // --- Import colors ---

    fn handle_import_colors(&mut self, code: KeyCode, is_cancel: bool) {
        if is_cancel {
            self.import_colors_open = false;
            return;
        }

        let user_themes = manager::list_themes().unwrap_or_default();
        let user_theme_data: Vec<_> = user_themes
            .iter()
            .filter_map(|(_, path)| manager::load_theme(path).ok())
            .collect();
        let schemes = super::widgets::import_menu::filter_color_schemes(&user_theme_data);
        let total = schemes.len() + user_themes.len();

        match code {
            KeyCode::Up => {
                if self.import_colors_selection > 0 {
                    self.import_colors_selection -= 1;
                }
            }
            KeyCode::Down => {
                if self.import_colors_selection + 1 < total {
                    self.import_colors_selection += 1;
                }
            }
            KeyCode::Enter => {
                self.import_colors_open = false;
                let idx = self.import_colors_selection;
                if idx < schemes.len() {
                    schemes[idx].apply_to(&mut self.theme.components);
                    self.status_message =
                        Some(format!("Imported colors from {}", schemes[idx].name));
                    self.mark_dirty();
                } else {
                    let theme_idx = idx - schemes.len();
                    if let Some((name, path)) = user_themes.get(theme_idx)
                        && let Ok(src_theme) = manager::load_theme(path)
                    {
                        // Copy only colors from source theme
                        for src_comp in &src_theme.components {
                            if let Some(dest) = self
                                .theme
                                .components
                                .iter_mut()
                                .find(|c| c.id == src_comp.id)
                            {
                                dest.colors = src_comp.colors.clone();
                                dest.styles = src_comp.styles.clone();
                            }
                        }
                        self.status_message = Some(format!("Imported colors from {}", name));
                        self.mark_dirty();
                    }
                }
            }
            _ => {}
        }
    }

    // --- Import icons ---

    fn handle_import_icons(&mut self, code: KeyCode, is_cancel: bool) {
        if is_cancel {
            self.import_icons_open = false;
            return;
        }

        let user_themes = manager::list_themes().unwrap_or_default();
        let user_theme_data: Vec<_> = user_themes
            .iter()
            .filter_map(|(_, path)| manager::load_theme(path).ok())
            .collect();
        let icon_sets = super::widgets::import_menu::filter_icon_sets(&user_theme_data);
        let total = icon_sets.len() + user_themes.len();

        match code {
            KeyCode::Up => {
                if self.import_icons_selection > 0 {
                    self.import_icons_selection -= 1;
                }
            }
            KeyCode::Down => {
                if self.import_icons_selection + 1 < total {
                    self.import_icons_selection += 1;
                }
            }
            KeyCode::Enter => {
                self.import_icons_open = false;
                let idx = self.import_icons_selection;
                if idx < icon_sets.len() {
                    icon_sets[idx].apply_to(&mut self.theme.components);
                    self.status_message =
                        Some(format!("Imported icons from {}", icon_sets[idx].name));
                    self.mark_dirty();
                } else {
                    let theme_idx = idx - icon_sets.len();
                    if let Some((name, path)) = user_themes.get(theme_idx)
                        && let Ok(src_theme) = manager::load_theme(path)
                    {
                        for src_comp in &src_theme.components {
                            if let Some(dest) = self
                                .theme
                                .components
                                .iter_mut()
                                .find(|c| c.id == src_comp.id)
                            {
                                dest.icon = src_comp.icon.clone();
                            }
                        }
                        self.status_message = Some(format!("Imported icons from {}", name));
                        self.mark_dirty();
                    }
                }
            }
            _ => {}
        }
    }

    // --- Open menu ---

    fn handle_open_menu(&mut self, code: KeyCode, is_cancel: bool) {
        if is_cancel {
            self.open_menu_open = false;
            return;
        }
        match code {
            KeyCode::Up => {
                if self.open_menu_selection > 0 {
                    self.open_menu_selection -= 1;
                }
            }
            KeyCode::Down => {
                if self.open_menu_selection + 1 < self.open_menu_themes.len() {
                    self.open_menu_selection += 1;
                }
            }
            KeyCode::Enter => {
                self.open_menu_open = false;
                let idx = self.open_menu_selection;
                if let Some((name, path)) = self.open_menu_themes.get(idx).cloned() {
                    if self.is_dirty {
                        // Store which theme to open, then ask to discard
                        self.confirm_dialog_open = true;
                        self.confirm_dialog_message = format!("Discard changes and open {}?", name);
                        self.confirm_dialog_action = ConfirmAction::DiscardAndOpen;
                        self.pending_open_theme = Some((name, path));
                    } else {
                        self.do_open_theme(&name, &path);
                    }
                }
            }
            _ => {}
        }
    }

    fn switch_theme(&mut self, delta: i32) {
        if self.theme_list.is_empty() {
            return;
        }
        let new_idx = (self.theme_list_index as i32 + delta)
            .rem_euclid(self.theme_list.len() as i32) as usize;
        if new_idx == self.theme_list_index {
            return;
        }
        if let Some((name, path)) = self.theme_list.get(new_idx).cloned() {
            if self.is_dirty {
                // Auto-save before switching
                self.action_save();
            }
            self.do_open_theme(&name, &path);
            self.theme_list_index = new_idx;
        }
    }

    fn do_open_theme(&mut self, name: &str, path: &std::path::Path) {
        match manager::load_theme(path) {
            Ok(theme) => {
                self.theme_name = name.to_string();
                self.theme_path = path.to_path_buf();
                self.theme = theme;
                self.is_dirty = false;
                self.selected_component = 0;
                self.selected_field = FieldSelection::Enabled;
                self.refresh_theme_list();
                self.status_message = Some(format!("Opened {}", name));
            }
            Err(e) => self.status_message = Some(format!("Load error: {}", e)),
        }
    }

    // --- Color picker ---

    fn open_color_picker(&mut self) {
        // Initialize from current color
        if let Some(comp) = self.theme.components.get(self.selected_component) {
            let current_color = match self.selected_field {
                FieldSelection::IconColor => comp.colors.icon.as_ref(),
                FieldSelection::TextColor => comp.colors.text.as_ref(),
                FieldSelection::BackgroundColor => comp.colors.background.as_ref(),
                _ => None,
            };
            self.color_picker = match current_color {
                Some(AnsiColor::Color16 { c16 }) => {
                    let mut state = ColorPickerState {
                        c16_selection: *c16,
                        ..Default::default()
                    };
                    state.mode.set(&ColorPickerMode::Color16);
                    state
                }
                Some(AnsiColor::Color256 { c256 }) => {
                    let mut state = ColorPickerState {
                        c256_selection: *c256,
                        ..Default::default()
                    };
                    state.mode.set(&ColorPickerMode::Color256);
                    state
                }
                Some(AnsiColor::Rgb { r, g, b }) => {
                    let mut state = ColorPickerState::default();
                    state.set_rgb(*r, *g, *b);
                    state.mode.set(&ColorPickerMode::Rgb);
                    state
                }
                None => ColorPickerState::default(),
            };
        }
        self.color_picker_open = true;
    }

    fn handle_color_picker(&mut self, code: KeyCode, modifiers: KeyModifiers, is_cancel: bool) {
        if is_cancel {
            self.color_picker_open = false;
            return;
        }
        match code {
            KeyCode::Tab => {
                self.color_picker.mode.move_next();
            }
            KeyCode::BackTab => {
                self.color_picker.mode.move_prev();
            }
            KeyCode::Up => match *self.color_picker.mode.current() {
                ColorPickerMode::Color16 => {
                    let sel = self.color_picker.c16_selection;
                    if !sel.is_multiple_of(8) {
                        self.color_picker.c16_selection = sel - 1;
                    }
                }
                ColorPickerMode::Color256 => {
                    self.color_picker.c256_selection =
                        self.color_picker.c256_selection.saturating_sub(1);
                }
                ColorPickerMode::Rgb => {
                    if self.color_picker.rgb_focus > 0 {
                        self.color_picker.rgb_focus -= 1;
                    }
                }
            },
            KeyCode::Down => match *self.color_picker.mode.current() {
                ColorPickerMode::Color16 => {
                    let sel = self.color_picker.c16_selection;
                    if sel % 8 < 7 {
                        self.color_picker.c16_selection = sel + 1;
                    }
                }
                ColorPickerMode::Color256 => {
                    self.color_picker.c256_selection =
                        self.color_picker.c256_selection.saturating_add(1);
                }
                ColorPickerMode::Rgb => {
                    if self.color_picker.rgb_focus < 2 {
                        self.color_picker.rgb_focus += 1;
                    }
                }
            },
            KeyCode::Left => match *self.color_picker.mode.current() {
                ColorPickerMode::Color16 => {
                    if self.color_picker.c16_selection >= 8 {
                        self.color_picker.c16_selection -= 8;
                    }
                }
                ColorPickerMode::Color256 => {
                    self.color_picker.c256_selection =
                        self.color_picker.c256_selection.saturating_sub(16);
                }
                ColorPickerMode::Rgb => {
                    let ta = &mut self.color_picker.rgb_textareas[self.color_picker.rgb_focus];
                    ta.input(crossterm::event::KeyEvent::new(code, modifiers));
                }
            },
            KeyCode::Right => match *self.color_picker.mode.current() {
                ColorPickerMode::Color16 => {
                    if self.color_picker.c16_selection < 8 {
                        self.color_picker.c16_selection += 8;
                    }
                }
                ColorPickerMode::Color256 => {
                    self.color_picker.c256_selection =
                        self.color_picker.c256_selection.saturating_add(16);
                }
                ColorPickerMode::Rgb => {
                    let ta = &mut self.color_picker.rgb_textareas[self.color_picker.rgb_focus];
                    ta.input(crossterm::event::KeyEvent::new(code, modifiers));
                }
            },
            KeyCode::Char(c)
                if c.is_ascii_digit() && self.color_picker.mode == ColorPickerMode::Rgb =>
            {
                let ta = &mut self.color_picker.rgb_textareas[self.color_picker.rgb_focus];
                let current_len = ta.lines().first().map(|s| s.len()).unwrap_or(0);
                if current_len < 3 {
                    ta.input(crossterm::event::KeyEvent::new(KeyCode::Char(c), modifiers));
                }
            }
            KeyCode::Backspace if self.color_picker.mode == ColorPickerMode::Rgb => {
                let ta = &mut self.color_picker.rgb_textareas[self.color_picker.rgb_focus];
                ta.input(crossterm::event::KeyEvent::new(
                    KeyCode::Backspace,
                    modifiers,
                ));
            }
            KeyCode::Delete if self.color_picker.mode == ColorPickerMode::Rgb => {
                let ta = &mut self.color_picker.rgb_textareas[self.color_picker.rgb_focus];
                ta.input(crossterm::event::KeyEvent::new(KeyCode::Delete, modifiers));
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                // Remove color (set to None)
                if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                    match self.selected_field {
                        FieldSelection::IconColor => comp.colors.icon = None,
                        FieldSelection::TextColor => comp.colors.text = None,
                        FieldSelection::BackgroundColor => comp.colors.background = None,
                        _ => {}
                    }
                    self.mark_dirty();
                }
                self.color_picker_open = false;
                self.status_message = Some("Color removed".into());
            }
            KeyCode::Enter => {
                let color = match *self.color_picker.mode.current() {
                    ColorPickerMode::Color16 => AnsiColor::Color16 {
                        c16: self.color_picker.c16_selection,
                    },
                    ColorPickerMode::Color256 => AnsiColor::Color256 {
                        c256: self.color_picker.c256_selection,
                    },
                    ColorPickerMode::Rgb => {
                        let r = self.color_picker.r_val();
                        let g = self.color_picker.g_val();
                        let b = self.color_picker.b_val();
                        AnsiColor::Rgb { r, g, b }
                    }
                };
                if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                    match self.selected_field {
                        FieldSelection::IconColor => comp.colors.icon = Some(color),
                        FieldSelection::TextColor => comp.colors.text = Some(color),
                        FieldSelection::BackgroundColor => comp.colors.background = Some(color),
                        _ => {}
                    }
                    self.mark_dirty();
                }
                self.color_picker_open = false;
                self.status_message = Some("Color updated".into());
            }
            _ => {}
        }
    }

    // --- Icon picker ---

    fn open_icon_picker(&mut self, purpose: IconPickerPurpose) {
        let is_nerd_style = matches!(
            self.theme.style.mode,
            StyleMode::NerdFont | StyleMode::Powerline
        );
        let initial_tab = match purpose {
            IconPickerPurpose::NerdFontIcon => IconPickerTab::NerdFont,
            IconPickerPurpose::FlashIcon
            | IconPickerPurpose::ProIcon
            | IconPickerPurpose::UltraIcon
            | IconPickerPurpose::FlashLiteIcon
            | IconPickerPurpose::OpusIcon
            | IconPickerPurpose::SonnetIcon
            | IconPickerPurpose::HaikuIcon
            | IconPickerPurpose::FableIcon
            | IconPickerPurpose::MythosIcon
                if is_nerd_style =>
            {
                IconPickerTab::NerdFont
            }
            _ => IconPickerTab::Emoji,
        };

        // Pre-fill custom buffer with the current icon value
        let current = if let Some(comp) = self.theme.components.get(self.selected_component) {
            match purpose {
                IconPickerPurpose::PlainIcon => comp.icon.plain.clone(),
                IconPickerPurpose::NerdFontIcon => comp.icon.nerd_font.clone(),
                IconPickerPurpose::ThinkingIcon => comp
                    .options
                    .get("thinking_icon")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                IconPickerPurpose::FlashIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.flash.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::ProIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.pro.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::UltraIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.ultra.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::FlashLiteIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.flash_lite.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::OpusIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.opus.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::SonnetIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.sonnet.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::HaikuIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.haiku.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::FableIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.fable.for_mode(self.theme.style.mode).to_string()
                    })
                }
                IconPickerPurpose::MythosIcon => {
                    comp.icon.per_model.as_ref().map_or(String::new(), |p| {
                        p.mythos.for_mode(self.theme.style.mode).to_string()
                    })
                }
            }
        } else {
            String::new()
        };

        let mut search_ta = TextArea::default();
        search_ta.set_cursor_line_style(Style::default());

        let mut custom_ta = TextArea::new(vec![current]);
        custom_ta.move_cursor(CursorMove::End);
        custom_ta.set_cursor_line_style(Style::default());

        self.icon_picker = IconPickerState {
            tab: RingCursor::new(vec![
                IconPickerTab::Emoji,
                IconPickerTab::NerdFont,
                IconPickerTab::Unicode,
                IconPickerTab::Custom,
            ]),
            purpose,
            search_textarea: search_ta,
            selected_index: 0,
            scroll_offset: 0,
            custom_textarea: custom_ta,
        };
        self.icon_picker.tab.set(&initial_tab);
        self.icon_picker_open = true;
    }

    pub fn handle_icon_picker(&mut self, code: KeyCode, is_cancel: bool) {
        self.handle_icon_picker_with_modifiers(code, KeyModifiers::NONE, is_cancel);
    }

    pub fn handle_icon_picker_with_modifiers(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
        is_cancel: bool,
    ) {
        if is_cancel {
            self.icon_picker_open = false;
            return;
        }

        let is_custom = *self.icon_picker.tab.current() == IconPickerTab::Custom;

        match code {
            KeyCode::Tab => {
                self.icon_picker.tab.move_next();
                self.icon_picker.selected_index = 0;
                self.icon_picker.scroll_offset = 0;
            }
            KeyCode::BackTab => {
                self.icon_picker.tab.move_prev();
                self.icon_picker.selected_index = 0;
                self.icon_picker.scroll_offset = 0;
            }
            KeyCode::Up if !is_custom => {
                if self.icon_picker.selected_index > 0 {
                    self.icon_picker.selected_index -= 1;
                    self.adjust_icon_picker_scroll();
                }
            }
            KeyCode::Down if !is_custom => {
                let max = self.icon_picker_selectable_count();
                if max > 0 && self.icon_picker.selected_index < max - 1 {
                    self.icon_picker.selected_index += 1;
                    self.adjust_icon_picker_scroll();
                }
            }
            KeyCode::PageUp if !is_custom => {
                let page = self.icon_picker_visible_height();
                self.icon_picker.selected_index =
                    self.icon_picker.selected_index.saturating_sub(page);
                self.adjust_icon_picker_scroll();
            }
            KeyCode::PageDown if !is_custom => {
                let page = self.icon_picker_visible_height();
                let max = self.icon_picker_selectable_count();
                if max > 0 {
                    self.icon_picker.selected_index =
                        (self.icon_picker.selected_index + page).min(max - 1);
                    self.adjust_icon_picker_scroll();
                }
            }
            KeyCode::Home if !is_custom => {
                self.icon_picker.selected_index = 0;
                self.adjust_icon_picker_scroll();
            }
            KeyCode::End if !is_custom => {
                let max = self.icon_picker_selectable_count();
                if max > 0 {
                    self.icon_picker.selected_index = max - 1;
                    self.adjust_icon_picker_scroll();
                }
            }
            KeyCode::Enter => {
                self.apply_icon_picker_selection();
            }
            _ => {
                let key_event = crossterm::event::KeyEvent::new(code, modifiers);
                if is_custom {
                    self.icon_picker.custom_textarea.input(key_event);
                } else {
                    let before = self.icon_picker.search_query().to_string();
                    self.icon_picker.search_textarea.input(key_event);
                    if self.icon_picker.search_query() != before {
                        self.icon_picker.selected_index = 0;
                        self.icon_picker.scroll_offset = 0;
                    }
                }
            }
        }
    }

    fn icon_picker_selectable_count(&self) -> usize {
        let tab = *self.icon_picker.tab.current();
        let sections = self
            .icon_catalog
            .sections(tab, self.icon_picker.search_query());
        super::widgets::icon_picker::selectable_count(&sections)
    }

    fn icon_picker_visible_height(&self) -> usize {
        // popup=26, outer border=2, tabs=3, search=3, keymap=3, icons border=2 → 13
        13
    }

    fn adjust_icon_picker_scroll(&mut self) {
        let visible = self.icon_picker_visible_height();
        let tab = *self.icon_picker.tab.current();
        let sections = self
            .icon_catalog
            .sections(tab, self.icon_picker.search_query());

        let Some(flat_idx) = super::widgets::icon_picker::selectable_to_flat(
            &sections,
            self.icon_picker.selected_index,
        ) else {
            return;
        };

        if flat_idx < self.icon_picker.scroll_offset {
            self.icon_picker.scroll_offset = flat_idx;
        } else if flat_idx >= self.icon_picker.scroll_offset + visible {
            self.icon_picker.scroll_offset = flat_idx.saturating_sub(visible - 1);
        }
    }

    fn apply_icon_picker_selection(&mut self) {
        let is_custom = *self.icon_picker.tab.current() == IconPickerTab::Custom;

        let icon_str = if is_custom {
            self.icon_picker.custom_buffer().to_string()
        } else {
            let tab = *self.icon_picker.tab.current();
            let sections = self
                .icon_catalog
                .sections(tab, self.icon_picker.search_query());

            match super::widgets::icon_picker::entry_at_selectable(
                &sections,
                self.icon_picker.selected_index,
            ) {
                Some(entry) => entry.icon.clone(),
                None => return, // nothing selected
            }
        };

        if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
            let label = match self.icon_picker.purpose {
                IconPickerPurpose::PlainIcon => {
                    comp.icon.plain = icon_str;
                    "Plain icon"
                }
                IconPickerPurpose::NerdFontIcon => {
                    comp.icon.nerd_font = icon_str;
                    "Nerd Font icon"
                }
                IconPickerPurpose::ThinkingIcon => {
                    comp.options
                        .insert("thinking_icon".into(), serde_json::Value::String(icon_str));
                    "Thinking icon"
                }
                IconPickerPurpose::FlashIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.flash.for_mode_mut(mode) = icon_str;
                    }
                    "Flash icon"
                }
                IconPickerPurpose::ProIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.pro.for_mode_mut(mode) = icon_str;
                    }
                    "Pro icon"
                }
                IconPickerPurpose::UltraIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.ultra.for_mode_mut(mode) = icon_str;
                    }
                    "Ultra icon"
                }
                IconPickerPurpose::FlashLiteIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.flash_lite.for_mode_mut(mode) = icon_str;
                    }
                    "Flash Lite icon"
                }
                IconPickerPurpose::OpusIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.opus.for_mode_mut(mode) = icon_str;
                    }
                    "Opus icon"
                }
                IconPickerPurpose::SonnetIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.sonnet.for_mode_mut(mode) = icon_str;
                    }
                    "Sonnet icon"
                }
                IconPickerPurpose::HaikuIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.haiku.for_mode_mut(mode) = icon_str;
                    }
                    "Haiku icon"
                }
                IconPickerPurpose::FableIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.fable.for_mode_mut(mode) = icon_str;
                    }
                    "Fable icon"
                }
                IconPickerPurpose::MythosIcon => {
                    let mode = self.theme.style.mode;
                    if let Some(pm) = comp.icon.per_model.as_mut() {
                        *pm.mythos.for_mode_mut(mode) = icon_str;
                    }
                    "Mythos icon"
                }
            };
            self.status_message = Some(format!("{} updated", label));
            self.mark_dirty();
        }
        self.icon_picker_open = false;
    }

    // --- UI rendering ---

    pub fn ui(&self, f: &mut Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};

        let height = f.area().height;
        let show_banner = height >= 25;
        let compact = height <= 21;

        let preview_height = if show_banner {
            super::widgets::banner::HEIGHT
        } else if compact {
            1
        } else {
            3
        };
        let spacer_height = if compact { 0 } else { 1 };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(preview_height), // Preview / Banner
                Constraint::Length(spacer_height),  // Spacer
                Constraint::Length(3),              // Themes bar
                Constraint::Min(3),                 // Main content (scrollable)
                Constraint::Length(3),              // Keymap
            ])
            .split(f.area());

        // Preview
        if show_banner {
            super::widgets::banner::render(f, layout[0], &self.theme);
        } else {
            PreviewWidget::render(f, layout[0], &self.theme, compact);
        }

        // Themes bar
        super::widgets::theme_bar::render(
            f,
            layout[2],
            &self.theme_list,
            self.theme_list_index,
            self.active_theme_name.as_deref(),
        );

        // Main content: two columns
        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(31), Constraint::Min(30)])
            .split(layout[3]);

        ComponentListWidget::render(
            f,
            content[0],
            &self.theme,
            self.selected_component,
            self.selected_panel == Panel::ComponentList,
        );

        EditorWidget::render(
            f,
            content[1],
            &self.theme,
            self.selected_component,
            self.selected_panel == Panel::Editor,
            self.selected_field,
        );

        // Keymap
        HelpBarWidget::render(f, layout[4]);

        // Popups (rendered on top)
        if self.file_menu_open {
            super::widgets::file_menu::render(f, f.area(), self.file_menu_selection.index());
        }
        if self.import_colors_open {
            super::widgets::import_menu::render_colors(
                f,
                f.area(),
                self.import_colors_selection,
                &self.theme,
            );
        }
        if self.import_icons_open {
            super::widgets::import_menu::render_icons(
                f,
                f.area(),
                self.import_icons_selection,
                &self.theme,
            );
        }
        if self.open_menu_open {
            super::widgets::open_menu::render(
                f,
                f.area(),
                &self.open_menu_themes,
                self.open_menu_selection,
            );
        }
        if self.name_input_open {
            let title = match self.name_input_purpose {
                NameInputPurpose::SaveAs => "Save As",
                NameInputPurpose::Rename => "Rename",
                NameInputPurpose::HostnameRstrip => "RStrip",
                NameInputPurpose::ModelSearch => "Search Regex",
                NameInputPurpose::ModelReplace => "Replacement",
            };
            super::widgets::name_input::render(f, f.area(), title, &self.name_input_textarea);
        }
        if self.confirm_dialog_open {
            super::widgets::confirm_dialog::render(
                f,
                f.area(),
                &self.confirm_dialog_message,
                self.confirm_dialog_action.hints(),
            );
        }
        if self.color_picker_open {
            super::widgets::color_picker::render(f, f.area(), &self.color_picker);
        }
        if self.icon_picker_open {
            super::widgets::icon_picker::render(f, f.area(), &self.icon_picker, &self.icon_catalog);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{App, FieldSelection, NameInputPurpose, Panel};
    use crate::config::theme::UserTheme;
    use crate::config::types::{
        ComponentId, GIT_OPTION_AUTOHIDE_BRANCH, MODEL_OPTION_REPLACE, MODEL_OPTION_SEARCH,
        MODEL_OPTION_SHOW_EFFORT, PR_OPTION_OSC_HYPERLINKS, PR_OPTION_SHOW_REVIEW_STATE,
        PR_OPTION_SHOW_URL, USAGE_OPTION_VALUE, WORKTREE_OPTION_OUTSIDE_WORKTREES,
        WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH,
    };
    use crossterm::event::KeyCode;

    #[test]
    fn pull_request_editor_toggles_each_option_independently() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );
        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::PullRequest)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);

        for (field, option, expected) in [
            (
                FieldSelection::PrReviewState,
                PR_OPTION_SHOW_REVIEW_STATE,
                false,
            ),
            (FieldSelection::PrUrl, PR_OPTION_SHOW_URL, true),
            (
                FieldSelection::PrOscHyperlinks,
                PR_OPTION_OSC_HYPERLINKS,
                false,
            ),
        ] {
            app.selected_field = field;
            app.toggle_current();
            assert_eq!(
                app.theme.components[app.selected_component]
                    .options
                    .get(option)
                    .and_then(|value| value.as_bool()),
                Some(expected)
            );
        }
    }

    #[test]
    fn worktree_editor_toggles_original_branch() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );
        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Worktree)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);
        app.selected_field = FieldSelection::WorktreeOriginalBranch;

        app.toggle_current();

        assert_eq!(
            app.theme.components[app.selected_component]
                .options
                .get(WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH)
                .and_then(|value| value.as_bool()),
            Some(true)
        );
    }

    #[test]
    fn worktree_editor_cycles_outside_worktree_modes() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );
        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Worktree)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);
        app.selected_field = FieldSelection::WorktreeOutside;

        for expected in ["directory", "hide", "show", "branch"] {
            app.toggle_current();
            assert_eq!(
                app.theme.components[app.selected_component]
                    .options
                    .get(WORKTREE_OPTION_OUTSIDE_WORKTREES)
                    .and_then(|value| value.as_str()),
                Some(expected)
            );
        }
    }

    #[test]
    fn git_status_editor_toggles_branch_autohide() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );
        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Git)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);
        app.selected_field = FieldSelection::GitAutohideBranch;

        app.toggle_current();

        assert_eq!(
            app.theme.components[app.selected_component]
                .options
                .get(GIT_OPTION_AUTOHIDE_BRANCH)
                .and_then(|value| value.as_bool()),
            Some(false)
        );
    }

    #[test]
    fn usage_editor_cycles_the_value_setting_per_component() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );

        for id in [ComponentId::UsageFiveHour, ComponentId::UsageSevenDay] {
            app.selected_component = app
                .theme
                .components
                .iter()
                .position(|component| component.id == id)
                .unwrap();
            app.selected_panel.set(&Panel::Editor);
            app.selected_field = FieldSelection::UsageValue;

            let value = |app: &App| {
                app.theme.components[app.selected_component]
                    .options
                    .get(USAGE_OPTION_VALUE)
                    .and_then(|value| value.as_str())
                    .map(str::to_owned)
            };

            app.toggle_current();
            assert_eq!(value(&app).as_deref(), Some("used"));
            app.toggle_current();
            assert_eq!(value(&app).as_deref(), Some("remaining"));
        }
    }

    #[test]
    fn model_editor_cycles_the_effort_setting() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );

        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Model)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);
        app.selected_field = FieldSelection::EffortLevel;

        let effort = |app: &App| {
            app.theme.components[app.selected_component]
                .options
                .get(MODEL_OPTION_SHOW_EFFORT)
                .and_then(|value| value.as_str())
                .map(str::to_owned)
        };

        // Default is "show"
        assert_eq!(effort(&app).as_deref(), Some("show"));
        app.toggle_current();
        assert_eq!(effort(&app).as_deref(), Some("gemini"));
        app.toggle_current();
        assert_eq!(effort(&app).as_deref(), Some("third_party"));
        app.toggle_current();
        assert_eq!(effort(&app).as_deref(), Some("hide"));
        app.toggle_current();
        assert_eq!(effort(&app).as_deref(), Some("show"));
    }

    #[test]
    fn model_editor_edits_search_and_replace() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );

        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Model)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);

        // Edit Search
        app.selected_field = FieldSelection::ModelSearch;
        app.toggle_current();
        assert!(app.name_input_open);
        assert_eq!(app.name_input_purpose, NameInputPurpose::ModelSearch);

        for c in "Gemini (\\d+)".chars() {
            app.handle_name_input(KeyCode::Char(c), false);
        }
        app.handle_name_input(KeyCode::Enter, false);
        assert!(!app.name_input_open);

        let search = app.theme.components[app.selected_component]
            .options
            .get(MODEL_OPTION_SEARCH)
            .and_then(|v| v.as_str())
            .unwrap();
        assert_eq!(search, "Gemini (\\d+)");

        // Edit Replace
        app.selected_field = FieldSelection::ModelReplace;
        app.toggle_current();
        assert!(app.name_input_open);
        assert_eq!(app.name_input_purpose, NameInputPurpose::ModelReplace);

        for c in "G-$1".chars() {
            app.handle_name_input(KeyCode::Char(c), false);
        }
        app.handle_name_input(KeyCode::Enter, false);
        assert!(!app.name_input_open);

        let replace = app.theme.components[app.selected_component]
            .options
            .get(MODEL_OPTION_REPLACE)
            .and_then(|v| v.as_str())
            .unwrap();
        assert_eq!(replace, "G-$1");
    }

    #[test]
    fn per_model_icons_update_per_style_mode() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );
        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Model)
            .unwrap();

        // StyleMode::Plain is default
        assert_eq!(app.theme.style.mode, crate::config::types::StyleMode::Plain);
        let model = &mut app.theme.components[app.selected_component];
        let pm = model.icon.per_model.as_mut().unwrap();
        pm.opus.plain = "🐙".into();
        pm.opus.nerd_font = "󰏒".into();

        // Pick icon in Plain mode updates plain variant
        app.theme.style.mode = crate::config::types::StyleMode::Plain;
        app.icon_picker.purpose = crate::tui::app::IconPickerPurpose::OpusIcon;
        app.icon_picker
            .tab
            .set(&crate::data::icon_catalog::IconPickerTab::Custom);
        app.icon_picker.set_custom_buffer("🦑");
        app.apply_icon_picker_selection();

        let model = &app.theme.components[app.selected_component];
        let pm = model.icon.per_model.as_ref().unwrap();
        assert_eq!(pm.opus.plain, "🦑");
        assert_eq!(pm.opus.nerd_font, "󰏒");

        // Pick icon in NerdFont mode updates nerd_font variant
        app.theme.style.mode = crate::config::types::StyleMode::NerdFont;
        app.icon_picker.purpose = crate::tui::app::IconPickerPurpose::OpusIcon;
        app.icon_picker
            .tab
            .set(&crate::data::icon_catalog::IconPickerTab::Custom);
        app.icon_picker.set_custom_buffer("🦅");
        app.apply_icon_picker_selection();

        let model = &app.theme.components[app.selected_component];
        let pm = model.icon.per_model.as_ref().unwrap();
        assert_eq!(pm.opus.plain, "🦑");
        assert_eq!(pm.opus.nerd_font, "🦅");
    }

    #[test]
    fn name_input_textarea_editing_and_submission() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );

        app.open_name_input(NameInputPurpose::HostnameRstrip, ".local");
        assert_eq!(app.name_input_buffer(), ".local");

        // Type additional suffix
        for c in ",.internal".chars() {
            app.handle_key(KeyCode::Char(c), crossterm::event::KeyModifiers::NONE);
        }
        assert_eq!(app.name_input_buffer(), ".local,.internal");

        // Press Backspace
        app.handle_key(KeyCode::Backspace, crossterm::event::KeyModifiers::NONE);
        assert_eq!(app.name_input_buffer(), ".local,.interna");

        // Submit with Enter
        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|c| c.id == ComponentId::Hostname)
            .unwrap();
        app.name_input_purpose = NameInputPurpose::HostnameRstrip;
        app.handle_key(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert!(!app.name_input_open);

        let rstrip = app.theme.components[app.selected_component]
            .options
            .get("rstrip")
            .and_then(|v| v.as_str());
        assert_eq!(rstrip, Some(".local,.interna"));
    }

    #[test]
    fn icon_picker_search_and_custom_textareas() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );

        app.open_icon_picker(crate::tui::app::IconPickerPurpose::PlainIcon);
        assert!(app.icon_picker_open);

        // Search textarea input
        for c in "rocket".chars() {
            app.handle_key(KeyCode::Char(c), crossterm::event::KeyModifiers::NONE);
        }
        assert_eq!(app.icon_picker.search_query(), "rocket");

        // Switch to custom tab
        app.icon_picker
            .tab
            .set(&crate::data::icon_catalog::IconPickerTab::Custom);
        app.icon_picker.set_custom_buffer("");

        for c in "🚀✨".chars() {
            app.handle_key(KeyCode::Char(c), crossterm::event::KeyModifiers::NONE);
        }
        assert_eq!(app.icon_picker.custom_buffer(), "🚀✨");

        // Apply custom icon
        app.handle_key(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert_eq!(app.theme.components[0].icon.plain, "🚀✨");
    }

    #[test]
    fn color_picker_rgb_textareas_editing() {
        let mut app = App::new(
            "Test".into(),
            "/tmp/Test.toml".into(),
            UserTheme::default_theme(),
        );

        app.selected_component = 0;
        app.selected_field = FieldSelection::TextColor;
        app.open_color_picker();
        assert!(app.color_picker_open);

        app.color_picker
            .mode
            .set(&crate::tui::app::ColorPickerMode::Rgb);
        app.color_picker.set_rgb(10, 20, 30);
        assert_eq!(app.color_picker.r_val(), 10);
        assert_eq!(app.color_picker.g_val(), 20);
        assert_eq!(app.color_picker.b_val(), 30);

        // Clear R field and type 255
        app.color_picker.rgb_focus = 0;
        app.color_picker.rgb_textareas[0] = ratatui_textarea::TextArea::new(vec![String::new()]);
        for c in "255".chars() {
            app.handle_key(KeyCode::Char(c), crossterm::event::KeyModifiers::NONE);
        }
        assert_eq!(app.color_picker.r_val(), 255);

        // Move to G field and edit
        app.handle_key(KeyCode::Down, crossterm::event::KeyModifiers::NONE);
        assert_eq!(app.color_picker.rgb_focus, 1);
        app.color_picker.rgb_textareas[1] = ratatui_textarea::TextArea::new(vec![String::new()]);
        for c in "100".chars() {
            app.handle_key(KeyCode::Char(c), crossterm::event::KeyModifiers::NONE);
        }
        assert_eq!(app.color_picker.g_val(), 100);

        // Submit color
        app.handle_key(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);
        assert!(!app.color_picker_open);
        assert_eq!(
            app.theme.components[0].colors.text,
            Some(crate::config::types::AnsiColor::Rgb {
                r: 255,
                g: 100,
                b: 30,
            })
        );
    }
}
