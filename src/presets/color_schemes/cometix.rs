use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use AnsiColor::Color16 as C;
    use ComponentId::*;

    ColorScheme::new(
        "Cometix",
        "Bold 16-color palette",
        vec![
            (
                AgentState,
                ComponentColors {
                    icon: Some(C { c16: 10 }),
                    text: Some(C { c16: 10 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Model,
                ComponentColors {
                    icon: Some(C { c16: 14 }),
                    text: Some(C { c16: 14 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: Some(C { c16: 11 }),
                    text: Some(C { c16: 10 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Quota,
                ComponentColors {
                    icon: Some(C { c16: 13 }),
                    text: Some(C { c16: 13 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                TaskCount,
                ComponentColors {
                    icon: Some(C { c16: 14 }),
                    text: Some(C { c16: 14 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ExecutionMode,
                ComponentColors {
                    icon: Some(C { c16: 6 }),
                    text: Some(C { c16: 6 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                VimMode,
                ComponentColors {
                    icon: Some(C { c16: 11 }),
                    text: Some(C { c16: 11 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ArtifactCount,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                PendingInput,
                ComponentColors {
                    icon: Some(C { c16: 13 }),
                    text: Some(C { c16: 13 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ToolConfirmation,
                ComponentColors {
                    icon: Some(C { c16: 9 }),
                    text: Some(C { c16: 9 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Sandbox,
                ComponentColors {
                    icon: Some(C { c16: 10 }),
                    text: Some(C { c16: 10 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                PlanTier,
                ComponentColors {
                    icon: Some(C { c16: 11 }),
                    text: Some(C { c16: 11 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Email,
                ComponentColors {
                    icon: Some(C { c16: 7 }),
                    text: Some(C { c16: 7 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Worktree,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                PullRequest,
                ComponentColors {
                    icon: Some(C { c16: 12 }),
                    text: Some(C { c16: 12 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: Some(C { c16: 13 }),
                    text: Some(C { c16: 13 }),
                    background: None,
                    text_bold: true,
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
                    text_bold: true,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(C { c16: 2 }),
                    text: Some(C { c16: 2 }),
                    background: None,
                    text_bold: true,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(C { c16: 6 }),
                    text: Some(C { c16: 6 }),
                    background: None,
                    text_bold: true,
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
