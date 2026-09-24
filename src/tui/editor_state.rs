use crate::config::theme::ResolvedTheme;
use crate::config::types::{
    AnsiColor, DEFAULT_GIT_AUTOHIDE_BRANCH, DEFAULT_HOSTNAME_RSTRIP, DEFAULT_PR_OSC_HYPERLINKS,
    DEFAULT_PR_SHOW_REVIEW_STATE, DEFAULT_PR_SHOW_URL, DEFAULT_WORKTREE_SHOW_ORIGINAL_BRANCH,
    GIT_OPTION_AUTOHIDE_BRANCH, MODEL_OPTION_REPLACE, MODEL_OPTION_SEARCH,
    MODEL_OPTION_SHOW_EFFORT, PR_OPTION_OSC_HYPERLINKS, PR_OPTION_SHOW_REVIEW_STATE,
    PR_OPTION_SHOW_URL, StyleMode, USAGE_OPTION_VALUE, UsageValue,
    WORKTREE_OPTION_OUTSIDE_WORKTREES, WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH, WorktreeOutside,
};
use crate::core::ring_cursor::RingCursor;
use crate::data::icon_catalog::{IconCatalogData, IconPickerTab};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::style::Style;
use ratatui_textarea::{CursorMove, TextArea};

use super::widgets::editor::FieldSelection;
use crate::config::catalog::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    ComponentList,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameInputPurpose {
    HostnameRstrip,
    ModelSearch,
    ModelReplace,
    PowerlinePlain,
    PowerlineNerd,
}

#[derive(Debug, Clone)]
pub struct ColorPickerState {
    pub error: Option<String>,
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
            error: None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorModal {
    Name,
    Color,
    Icon,
}

pub struct EditorState {
    pub theme: ResolvedTheme,
    pub kind: Kind,
    pub changed: bool,
    pub selected_component: usize,
    pub selected_panel: RingCursor<Panel>,
    pub selected_field: FieldSelection,
    pub status_message: Option<String>,
    pub modal: Option<EditorModal>,
    pub name_input_textarea: TextArea<'static>,
    pub name_input_purpose: NameInputPurpose,
    pub color_picker: ColorPickerState,
    pub icon_picker: IconPickerState,
    pub icon_catalog: IconCatalogData,
    pub icon_viewport: usize,
}

impl EditorState {
    pub fn new(theme: ResolvedTheme) -> Self {
        Self {
            theme,
            kind: Kind::Components,
            changed: false,
            selected_component: 0,
            selected_panel: RingCursor::new(vec![Panel::ComponentList, Panel::Editor]),
            selected_field: FieldSelection::Enabled,
            status_message: None,
            modal: None,
            name_input_textarea: TextArea::default(),
            name_input_purpose: NameInputPurpose::HostnameRstrip,
            color_picker: ColorPickerState::default(),
            icon_picker: IconPickerState::default(),
            icon_catalog: IconCatalogData::load(),
            icon_viewport: 1,
        }
    }

    pub fn fields(&self) -> Vec<FieldSelection> {
        let Some(c) = self.theme.components.get(self.selected_component) else {
            return Vec::new();
        };
        FieldSelection::fields_for_kind(c, self.kind)
    }

    pub fn clamp_field(&mut self) {
        let fields = self.fields();
        if !fields.contains(&self.selected_field) {
            self.selected_field = fields.first().copied().unwrap_or(FieldSelection::Enabled);
        }
    }

    pub fn has_modal(&self) -> bool {
        self.modal.is_some()
    }

    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        if self.modal == Some(EditorModal::Name) {
            self.handle_name_input_with_modifiers(code, modifiers, code == KeyCode::Esc);
            return;
        }
        if self.modal == Some(EditorModal::Color) {
            self.handle_color_picker(code, modifiers, code == KeyCode::Esc);
            return;
        }
        if self.modal == Some(EditorModal::Icon) {
            self.handle_icon_picker_with_modifiers(code, modifiers, code == KeyCode::Esc);
            return;
        }
        match code {
            KeyCode::Tab | KeyCode::BackTab | KeyCode::Left | KeyCode::Right => {
                self.selected_panel.move_next();
            }
            KeyCode::Up
                if modifiers.contains(KeyModifiers::SHIFT) && self.kind == Kind::Components =>
            {
                self.move_component_up()
            }
            KeyCode::Down
                if modifiers.contains(KeyModifiers::SHIFT) && self.kind == Kind::Components =>
            {
                self.move_component_down()
            }
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Down => self.move_selection(1),
            KeyCode::Enter | KeyCode::Char(' ') => self.toggle_current(),
            _ => {}
        }
        self.clamp_field();
    }

    pub fn mark_dirty(&mut self) {
        self.changed = true;
    }

    fn reorderable_count(&self) -> usize {
        self.theme.components.len()
    }

    fn move_selection(&mut self, delta: i32) {
        if self.selected_panel == Panel::ComponentList {
            self.selected_component = (self.selected_component as i32 + delta)
                .clamp(0, self.theme.components.len().saturating_sub(1) as i32)
                as usize;
            self.clamp_field();
        } else {
            let fields = self.fields();
            if fields.is_empty() {
                return;
            }
            let i = fields
                .iter()
                .position(|f| *f == self.selected_field)
                .unwrap_or(0);
            self.selected_field =
                fields[(i as i32 + delta).clamp(0, fields.len().saturating_sub(1) as i32) as usize];
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
        self.modal = Some(EditorModal::Name);
    }

    fn toggle_current(&mut self) {
        match *self.selected_panel.current() {
            Panel::ComponentList => {
                if self.kind != Kind::Components {
                    self.selected_panel.set(&Panel::Editor);
                    return;
                }
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
                        FieldSelection::GitShowSha => {
                            let comp = &mut self.theme.components[self.selected_component];
                            let value = comp
                                .options
                                .get("show_sha")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            comp.options.insert("show_sha".into(), (!value).into());
                            self.mark_dirty();
                        }
                        FieldSelection::GlyphMode => {
                            self.theme.style.mode = match self.theme.style.mode {
                                StyleMode::Plain => StyleMode::NerdFont,
                                StyleMode::NerdFont => StyleMode::Plain,
                                StyleMode::Powerline => StyleMode::PlainPowerline,
                                StyleMode::PlainPowerline => StyleMode::Powerline,
                            };
                            self.mark_dirty();
                        }
                        FieldSelection::Powerline => {
                            self.theme.style.mode = match self.theme.style.mode {
                                StyleMode::Plain => StyleMode::PlainPowerline,
                                StyleMode::PlainPowerline => StyleMode::Plain,
                                StyleMode::Powerline => StyleMode::NerdFont,
                                StyleMode::NerdFont => StyleMode::Powerline,
                            };
                            self.mark_dirty();
                        }
                        FieldSelection::PowerlinePlain => self.open_name_input(
                            NameInputPurpose::PowerlinePlain,
                            &self.theme.powerline_plain.clone(),
                        ),
                        FieldSelection::PowerlineNerd => self.open_name_input(
                            NameInputPurpose::PowerlineNerd,
                            &self.theme.powerline_nerd_font.clone(),
                        ),
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
        self.modal = Some(EditorModal::Color);
    }

    fn handle_color_picker(&mut self, code: KeyCode, modifiers: KeyModifiers, is_cancel: bool) {
        if is_cancel {
            self.modal = None;
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
                self.modal = None;
                self.status_message = Some("Color removed".into());
            }
            KeyCode::Enter => {
                if self.color_picker.mode == ColorPickerMode::Rgb
                    && self.color_picker.rgb_textareas.iter().any(|ta| {
                        ta.lines()
                            .first()
                            .and_then(|s| s.parse::<u8>().ok())
                            .is_none()
                    })
                {
                    self.color_picker.error =
                        Some("RGB values must be integers from 0 to 255".into());
                    return;
                }
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
                self.modal = None;
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
        self.modal = Some(EditorModal::Icon);
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
            self.modal = None;
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
                let page = self.icon_viewport.max(1);
                self.icon_picker.selected_index =
                    self.icon_picker.selected_index.saturating_sub(page);
                self.adjust_icon_picker_scroll();
            }
            KeyCode::PageDown if !is_custom => {
                let page = self.icon_viewport.max(1);
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

    pub fn adjust_icon_picker_scroll(&mut self) {
        let visible = self.icon_viewport.max(1);
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
        self.modal = None;
    }

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
            self.modal = None;
            return;
        }
        if code == KeyCode::Enter {
            let value = self.name_input_buffer();
            if self.name_input_purpose == NameInputPurpose::PowerlinePlain {
                self.theme.powerline_plain = value;
                self.mark_dirty();
            } else if self.name_input_purpose == NameInputPurpose::PowerlineNerd {
                self.theme.powerline_nerd_font = value;
                self.mark_dirty();
            } else if let Some(comp) = self.theme.components.get_mut(self.selected_component) {
                let key = match self.name_input_purpose {
                    NameInputPurpose::HostnameRstrip => "rstrip",
                    NameInputPurpose::ModelSearch => MODEL_OPTION_SEARCH,
                    NameInputPurpose::ModelReplace => MODEL_OPTION_REPLACE,
                    _ => unreachable!(),
                };
                comp.options.insert(key.into(), value.into());
                self.mark_dirty();
            }
            self.modal = None;
        } else {
            self.name_input_textarea
                .input(crossterm::event::KeyEvent::new(code, modifiers));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{EditorState, FieldSelection, NameInputPurpose, Panel};
    use crate::config::theme::ResolvedTheme;
    use crate::config::types::{
        ComponentId, GIT_OPTION_AUTOHIDE_BRANCH, MODEL_OPTION_REPLACE, MODEL_OPTION_SEARCH,
        MODEL_OPTION_SHOW_EFFORT, PR_OPTION_OSC_HYPERLINKS, PR_OPTION_SHOW_REVIEW_STATE,
        PR_OPTION_SHOW_URL, USAGE_OPTION_VALUE, WORKTREE_OPTION_OUTSIDE_WORKTREES,
        WORKTREE_OPTION_SHOW_ORIGINAL_BRANCH,
    };
    use crossterm::event::KeyCode;

    #[test]
    fn pull_request_editor_toggles_each_option_independently() {
        let mut app = EditorState::new(ResolvedTheme::default_theme());
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());

        for id in [ComponentId::UsageFiveHour, ComponentId::UsageSevenDay] {
            app.selected_component = app
                .theme
                .components
                .iter()
                .position(|component| component.id == id)
                .unwrap();
            app.selected_panel.set(&Panel::Editor);
            app.selected_field = FieldSelection::UsageValue;

            let value = |app: &EditorState| {
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());

        app.selected_component = app
            .theme
            .components
            .iter()
            .position(|component| component.id == ComponentId::Model)
            .unwrap();
        app.selected_panel.set(&Panel::Editor);
        app.selected_field = FieldSelection::EffortLevel;

        let effort = |app: &EditorState| {
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());

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
        assert!((app.modal == Some(super::EditorModal::Name)));
        assert_eq!(app.name_input_purpose, NameInputPurpose::ModelSearch);

        for c in "Gemini (\\d+)".chars() {
            app.handle_name_input(KeyCode::Char(c), false);
        }
        app.handle_name_input(KeyCode::Enter, false);
        assert!(!(app.modal == Some(super::EditorModal::Name)));

        let search = app.theme.components[app.selected_component]
            .options
            .get(MODEL_OPTION_SEARCH)
            .and_then(|v| v.as_str())
            .unwrap();
        assert_eq!(search, "Gemini (\\d+)");

        // Edit Replace
        app.selected_field = FieldSelection::ModelReplace;
        app.toggle_current();
        assert!((app.modal == Some(super::EditorModal::Name)));
        assert_eq!(app.name_input_purpose, NameInputPurpose::ModelReplace);

        for c in "G-$1".chars() {
            app.handle_name_input(KeyCode::Char(c), false);
        }
        app.handle_name_input(KeyCode::Enter, false);
        assert!(!(app.modal == Some(super::EditorModal::Name)));

        let replace = app.theme.components[app.selected_component]
            .options
            .get(MODEL_OPTION_REPLACE)
            .and_then(|v| v.as_str())
            .unwrap();
        assert_eq!(replace, "G-$1");
    }

    #[test]
    fn per_model_icons_update_per_style_mode() {
        let mut app = EditorState::new(ResolvedTheme::default_theme());
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
        app.icon_picker.purpose = crate::tui::editor_state::IconPickerPurpose::OpusIcon;
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
        app.icon_picker.purpose = crate::tui::editor_state::IconPickerPurpose::OpusIcon;
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
        let mut app = EditorState::new(ResolvedTheme::default_theme());

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
        assert!(!(app.modal == Some(super::EditorModal::Name)));

        let rstrip = app.theme.components[app.selected_component]
            .options
            .get("rstrip")
            .and_then(|v| v.as_str());
        assert_eq!(rstrip, Some(".local,.interna"));
    }

    #[test]
    fn icon_picker_search_and_custom_textareas() {
        let mut app = EditorState::new(ResolvedTheme::default_theme());

        app.open_icon_picker(crate::tui::editor_state::IconPickerPurpose::PlainIcon);
        assert!((app.modal == Some(super::EditorModal::Icon)));

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
        let mut app = EditorState::new(ResolvedTheme::default_theme());

        app.selected_component = 0;
        app.selected_field = FieldSelection::TextColor;
        app.open_color_picker();
        assert!((app.modal == Some(super::EditorModal::Color)));

        app.color_picker
            .mode
            .set(&crate::tui::editor_state::ColorPickerMode::Rgb);
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
        assert!(!(app.modal == Some(super::EditorModal::Color)));
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

#[cfg(test)]
mod validation_tests {
    use super::*;
    #[test]
    fn invalid_rgb_entry_stays_open_without_changing_the_draft() {
        let mut state = EditorState::new(ResolvedTheme::default_theme());
        state.selected_field = FieldSelection::IconColor;
        let before = state.theme.components[0].colors.icon.clone();
        state.open_color_picker();
        state.color_picker.mode.set(&ColorPickerMode::Rgb);
        state.color_picker.rgb_textareas[0] = TextArea::new(vec!["999".into()]);
        state.handle_key(KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(state.modal, Some(EditorModal::Color));
        assert!(!state.changed);
        assert_eq!(state.theme.components[0].colors.icon, before);
        assert!(state.color_picker.error.is_some());
    }
}
