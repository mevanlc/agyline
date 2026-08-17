use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use AnsiColor::*;
    use ComponentId::*;

    ColorScheme::new(
        "Gruvbox",
        "Retro groove dark colors",
        vec![
            (
                AgentState,
                ComponentColors {
                    icon: Some(Color256 { c256: 142 }),
                    text: Some(Color256 { c256: 142 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Model,
                ComponentColors {
                    icon: Some(Color256 { c256: 208 }),
                    text: Some(Color256 { c256: 208 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: Some(Color256 { c256: 142 }),
                    text: Some(Color256 { c256: 142 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                TaskCount,
                ComponentColors {
                    icon: Some(Color256 { c256: 214 }),
                    text: Some(Color256 { c256: 214 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ExecutionMode,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                VimMode,
                ComponentColors {
                    icon: Some(Color256 { c256: 214 }),
                    text: Some(Color256 { c256: 214 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ArtifactCount,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                PendingInput,
                ComponentColors {
                    icon: Some(Color256 { c256: 175 }),
                    text: Some(Color256 { c256: 175 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ToolConfirmation,
                ComponentColors {
                    icon: Some(Color256 { c256: 167 }),
                    text: Some(Color256 { c256: 167 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Sandbox,
                ComponentColors {
                    icon: Some(Color256 { c256: 142 }),
                    text: Some(Color256 { c256: 142 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                PlanTier,
                ComponentColors {
                    icon: Some(Color256 { c256: 214 }),
                    text: Some(Color256 { c256: 214 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Email,
                ComponentColors {
                    icon: Some(Color256 { c256: 223 }),
                    text: Some(Color256 { c256: 223 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Worktree,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                PullRequest,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: Some(Color16 { c16: 5 }),
                    text: Some(Color16 { c16: 5 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                UsageFiveHour,
                ComponentColors {
                    icon: Some(Color16 { c16: 14 }),
                    text: Some(Color16 { c16: 14 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                UsageSevenDay,
                ComponentColors {
                    icon: Some(Color16 { c16: 14 }),
                    text: Some(Color16 { c16: 14 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: Some(Color256 { c256: 214 }),
                    text: Some(Color256 { c256: 214 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(Color256 { c256: 142 }),
                    text: Some(Color256 { c256: 142 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(Color256 { c256: 109 }),
                    text: Some(Color256 { c256: 109 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(Color256 { c256: 245 }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
