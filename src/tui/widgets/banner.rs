use crate::config::theme::UserTheme;
use crate::core::render;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

/// Colors matching the official Antigravity CLI terminal output.
const COLOR_TITLE: Color = Color::Rgb(138, 180, 248);
const COLOR_MUTED: Color = Color::Rgb(154, 160, 166);
const COLOR_BORDER: Color = Color::Rgb(60, 64, 67);
const COLOR_PROMPT: Color = Color::Rgb(138, 180, 248);

/// Try to read the Antigravity CLI version.
fn read_cli_version() -> String {
    if let Some(h) = dirs::home_dir() {
        let agy_path = h
            .join(".gemini")
            .join("antigravity-cli")
            .join("version.json");
        if let Ok(contents) = std::fs::read_to_string(&agy_path)
            && let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents)
            && let Some(s) = json.get("version").and_then(|v| v.as_str())
            && !s.is_empty()
        {
            return s.to_string();
        }
    }
    "1.1.13".into()
}

/// Detect the user account and subscription tier.
fn read_user_info() -> String {
    if let Ok(output) = std::process::Command::new("git")
        .args(["config", "user.email"])
        .output()
        && output.status.success()
    {
        let email = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !email.is_empty() {
            return format!("{} (Google AI Pro)", email);
        }
    }
    "you@example.com (Google AI Pro)".into()
}

/// Format current working directory with home abbreviation.
fn current_dir_display() -> String {
    if let Ok(cwd) = std::env::current_dir() {
        if let Some(home) = dirs::home_dir()
            && let Ok(suffix) = cwd.strip_prefix(&home)
        {
            return format!("~/{}", suffix.display());
        }
        return cwd.display().to_string();
    }
    "~/p/my/agyline".into()
}

/// Render the official Antigravity CLI header with Gaussian rainbow mascot,
/// user/model metadata, prompt horizontal rules, and live statusline preview.
pub fn render(f: &mut Frame, area: Rect, theme: &UserTheme) {
    let version = read_cli_version();
    let user_info = read_user_info();
    let cwd = current_dir_display();
    let width = area.width as usize;

    let texts = render::demo_texts_for_components(&theme.components);
    let statusline_line = render::build_render_line(&theme.components, theme.style.mode, &texts);
    let statusline_spans = render::render_spans(&statusline_line);

    let lines = vec![
        // Top margin
        Line::from(""),
        // Row 0: Gaussian Row 1 + Title
        Line::from(vec![
            Span::raw("      "),
            Span::styled("▄", Style::default().fg(Color::Rgb(219, 177, 49))),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(242, 146, 46))
                    .bg(Color::Rgb(246, 145, 46)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(240, 114, 54))
                    .bg(Color::Rgb(243, 115, 55)),
            ),
            Span::styled("▄", Style::default().fg(Color::Rgb(240, 88, 59))),
            Span::raw("        "),
            Span::styled(
                format!("Antigravity CLI {}", version),
                Style::default()
                    .fg(COLOR_TITLE)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        // Row 1: Gaussian Row 2 + Account
        Line::from(vec![
            Span::raw("     "),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(158, 195, 69))
                    .bg(Color::Rgb(134, 198, 78)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(181, 180, 62))
                    .bg(Color::Rgb(117, 180, 94)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(226, 153, 61))
                    .bg(Color::Rgb(204, 149, 77)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(246, 122, 52))
                    .bg(Color::Rgb(239, 121, 71)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(248, 106, 53))
                    .bg(Color::Rgb(225, 102, 82)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(239, 84, 66))
                    .bg(Color::Rgb(225, 79, 89)),
            ),
            Span::raw("       "),
            Span::styled(user_info, Style::default().fg(COLOR_MUTED)),
        ]),
        // Row 2: Gaussian Row 3 + Model
        Line::from(vec![
            Span::raw("    "),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(124, 194, 81))
                    .bg(Color::Rgb(128, 198, 84)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(113, 194, 92))
                    .bg(Color::Rgb(84, 184, 129)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(92, 169, 143))
                    .bg(Color::Rgb(64, 151, 222)),
            ),
            Span::styled("▀", Style::default().fg(Color::Rgb(92, 145, 179))),
            Span::styled("▀", Style::default().fg(Color::Rgb(131, 115, 176))),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(116, 111, 195))
                    .bg(Color::Rgb(74, 126, 228)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(153, 93, 168))
                    .bg(Color::Rgb(112, 110, 206)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(156, 91, 151))
                    .bg(Color::Rgb(143, 100, 180)),
            ),
            Span::raw("      "),
            Span::styled("Gemini 3.7 Flash (High)", Style::default().fg(COLOR_MUTED)),
        ]),
        // Row 3: Gaussian Row 4 + CWD
        Line::from(vec![
            Span::raw("   "),
            Span::styled("▄", Style::default().fg(Color::Rgb(109, 198, 148))),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(97, 195, 125))
                    .bg(Color::Rgb(98, 186, 213)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(67, 174, 171))
                    .bg(Color::Rgb(71, 168, 220)),
            ),
            Span::raw("    "),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(74, 128, 234))
                    .bg(Color::Rgb(61, 137, 251)),
            ),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(108, 115, 216))
                    .bg(Color::Rgb(74, 129, 240)),
            ),
            Span::styled("▄", Style::default().fg(Color::Rgb(101, 121, 225))),
            Span::raw("     "),
            Span::styled(cwd, Style::default().fg(COLOR_MUTED)),
        ]),
        // Row 4: Gaussian Row 5 (Feet)
        Line::from(vec![
            Span::raw("  "),
            Span::styled("▄", Style::default().fg(Color::Rgb(103, 185, 244))),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(107, 199, 163))
                    .bg(Color::Rgb(100, 182, 246)),
            ),
            Span::styled("▀", Style::default().fg(Color::Rgb(100, 182, 246))),
            Span::raw("      "),
            Span::styled("▀", Style::default().fg(Color::Rgb(56, 134, 251))),
            Span::styled(
                "▀",
                Style::default()
                    .fg(Color::Rgb(72, 129, 244))
                    .bg(Color::Rgb(56, 131, 249)),
            ),
            Span::styled("▄", Style::default().fg(Color::Rgb(61, 133, 252))),
        ]),
        // Row 5: Blank line
        Line::from(""),
        // Row 6: Top rule
        Line::from(Span::styled(
            "─".repeat(width),
            Style::default().fg(COLOR_BORDER),
        )),
        // Row 7: Prompt
        Line::from(Span::styled(">", Style::default().fg(COLOR_PROMPT))),
        // Row 8: Bottom rule
        Line::from(Span::styled(
            "─".repeat(width),
            Style::default().fg(COLOR_BORDER),
        )),
        // Row 9: Live statusline
        Line::from(statusline_spans),
    ];

    let widget = Paragraph::new(Text::from(lines));
    f.render_widget(widget, area);
}

/// Height needed for the banner + statusline.
pub const HEIGHT: u16 = 11;
