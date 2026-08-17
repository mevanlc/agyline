use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use ComponentId::*;

    ColorScheme::new(
        "Tokyo Night",
        "Modern dark palette with backgrounds",
        vec![
            (
                AgentState,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 158,
                        g: 206,
                        b: 106,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 158,
                        g: 206,
                        b: 106,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 41,
                        g: 46,
                        b: 66,
                    }),
                    text_bold: false,
                },
            ),
            (
                Model,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 252,
                        g: 167,
                        b: 234,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 252,
                        g: 167,
                        b: 234,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 25,
                        g: 27,
                        b: 41,
                    }),
                    text_bold: false,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 130,
                        g: 170,
                        b: 255,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 130,
                        g: 170,
                        b: 255,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 47,
                        g: 51,
                        b: 77,
                    }),
                    text_bold: false,
                },
            ),
            (
                TaskCount,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 36,
                        g: 40,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                ExecutionMode,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 125,
                        g: 207,
                        b: 255,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 125,
                        g: 207,
                        b: 255,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 32,
                        g: 35,
                        b: 52,
                    }),
                    text_bold: false,
                },
            ),
            (
                VimMode,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 36,
                        g: 40,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                ArtifactCount,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 130,
                        g: 170,
                        b: 255,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 130,
                        g: 170,
                        b: 255,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 30,
                        g: 32,
                        b: 48,
                    }),
                    text_bold: false,
                },
            ),
            (
                PendingInput,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 187,
                        g: 154,
                        b: 247,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 187,
                        g: 154,
                        b: 247,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 47,
                        g: 51,
                        b: 77,
                    }),
                    text_bold: false,
                },
            ),
            (
                ToolConfirmation,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 247,
                        g: 118,
                        b: 142,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 247,
                        g: 118,
                        b: 142,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 41,
                        g: 46,
                        b: 66,
                    }),
                    text_bold: false,
                },
            ),
            (
                Sandbox,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 158,
                        g: 206,
                        b: 106,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 158,
                        g: 206,
                        b: 106,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 41,
                        g: 46,
                        b: 66,
                    }),
                    text_bold: false,
                },
            ),
            (
                PlanTier,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 36,
                        g: 40,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                Email,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 192,
                        g: 202,
                        b: 245,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 192,
                        g: 202,
                        b: 245,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 32,
                        g: 35,
                        b: 52,
                    }),
                    text_bold: false,
                },
            ),
            (
                Worktree,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 195,
                        g: 232,
                        b: 141,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 195,
                        g: 232,
                        b: 141,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 30,
                        g: 32,
                        b: 48,
                    }),
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 125,
                        g: 207,
                        b: 255,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 125,
                        g: 207,
                        b: 255,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 32,
                        g: 35,
                        b: 52,
                    }),
                    text_bold: false,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 195,
                        g: 232,
                        b: 141,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 195,
                        g: 232,
                        b: 141,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 30,
                        g: 32,
                        b: 48,
                    }),
                    text_bold: false,
                },
            ),
            (
                PullRequest,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 195,
                        g: 232,
                        b: 141,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 195,
                        g: 232,
                        b: 141,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 30,
                        g: 32,
                        b: 48,
                    }),
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 192,
                        g: 202,
                        b: 245,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 192,
                        g: 202,
                        b: 245,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 61,
                        g: 89,
                        b: 161,
                    }),
                    text_bold: false,
                },
            ),
            (
                UsageFiveHour,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 36,
                        g: 40,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                UsageSevenDay,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 36,
                        g: 40,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 175,
                        b: 104,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 36,
                        g: 40,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 158,
                        g: 206,
                        b: 106,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 158,
                        g: 206,
                        b: 106,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 41,
                        g: 46,
                        b: 66,
                    }),
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 125,
                        g: 207,
                        b: 255,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 125,
                        g: 207,
                        b: 255,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 32,
                        g: 35,
                        b: 52,
                    }),
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 86,
                        g: 95,
                        b: 137,
                    }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
