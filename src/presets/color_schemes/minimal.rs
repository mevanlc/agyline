use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use AnsiColor::Color16 as C;
    use ComponentId::*;

    ColorScheme::new(
        "Minimal",
        "Subdued 16-color palette",
        vec![
            (
                AgentState,
                ComponentColors {
                    icon: Some(C { c16: 10 }),
                    text: Some(C { c16: 10 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Model,
                ComponentColors {
                    icon: Some(C { c16: 14 }),
                    text: Some(C { c16: 14 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: Some(C { c16: 11 }),
                    text: Some(C { c16: 10 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                TaskCount,
                ComponentColors {
                    icon: Some(C { c16: 14 }),
                    text: Some(C { c16: 14 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                ExecutionMode,
                ComponentColors {
                    icon: Some(C { c16: 6 }),
                    text: Some(C { c16: 6 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                VimMode,
                ComponentColors {
                    icon: Some(C { c16: 11 }),
                    text: Some(C { c16: 11 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                ArtifactCount,
                ComponentColors {
                    icon: Some(C { c16: 8 }),
                    text: Some(C { c16: 8 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                PendingInput,
                ComponentColors {
                    icon: Some(C { c16: 13 }),
                    text: Some(C { c16: 13 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                ToolConfirmation,
                ComponentColors {
                    icon: Some(C { c16: 9 }),
                    text: Some(C { c16: 9 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Sandbox,
                ComponentColors {
                    icon: Some(C { c16: 10 }),
                    text: Some(C { c16: 10 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                PlanTier,
                ComponentColors {
                    icon: Some(C { c16: 11 }),
                    text: Some(C { c16: 11 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Email,
                ComponentColors {
                    icon: Some(C { c16: 8 }),
                    text: Some(C { c16: 8 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Worktree,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: Some(C { c16: 8 }),
                    text: Some(C { c16: 8 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                PullRequest,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: Some(C { c16: 13 }),
                    text: Some(C { c16: 13 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                UsageFiveHour,
                ComponentColors {
                    icon: Some(C { c16: 14 }),
                    text: Some(C { c16: 14 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                UsageSevenDay,
                ComponentColors {
                    icon: Some(C { c16: 14 }),
                    text: Some(C { c16: 14 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: Some(C { c16: 3 }),
                    text: Some(C { c16: 3 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(C { c16: 2 }),
                    text: Some(C { c16: 2 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(C { c16: 6 }),
                    text: Some(C { c16: 6 }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(C { c16: 8 }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
