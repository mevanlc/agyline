use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use ComponentId::*;

    let dark = || {
        Some(AnsiColor::Rgb {
            r: 46,
            g: 52,
            b: 64,
        })
    };

    ColorScheme::new(
        "Nord",
        "Cool northern palette with backgrounds",
        vec![
            (
                Model,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 136,
                        g: 192,
                        b: 208,
                    }),
                    text_bold: false,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 163,
                        g: 190,
                        b: 140,
                    }),
                    text_bold: false,
                },
            ),
            (
                Worktree,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 163,
                        g: 190,
                        b: 140,
                    }),
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 94,
                        g: 129,
                        b: 172,
                    }),
                    text_bold: false,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 129,
                        g: 161,
                        b: 193,
                    }),
                    text_bold: false,
                },
            ),
            (
                PullRequest,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 129,
                        g: 161,
                        b: 193,
                    }),
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 180,
                        g: 142,
                        b: 173,
                    }),
                    text_bold: false,
                },
            ),
            (
                UsageFiveHour,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 235,
                        g: 203,
                        b: 139,
                    }),
                    text_bold: false,
                },
            ),
            (
                UsageSevenDay,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 235,
                        g: 203,
                        b: 139,
                    }),
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 235,
                        g: 203,
                        b: 139,
                    }),
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 163,
                        g: 190,
                        b: 140,
                    }),
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: dark(),
                    text: dark(),
                    background: Some(AnsiColor::Rgb {
                        r: 136,
                        g: 192,
                        b: 208,
                    }),
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 76,
                        g: 86,
                        b: 106,
                    }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
