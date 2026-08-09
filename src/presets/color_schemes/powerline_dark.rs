use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use ComponentId::*;

    let white = || {
        Some(AnsiColor::Rgb {
            r: 255,
            g: 255,
            b: 255,
        })
    };
    let light = || {
        Some(AnsiColor::Rgb {
            r: 209,
            g: 213,
            b: 219,
        })
    };

    ColorScheme::new(
        "Powerline Dark",
        "Dark powerline with backgrounds",
        vec![
            (
                Model,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 45,
                        g: 45,
                        b: 45,
                    }),
                    text_bold: false,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 139,
                        g: 69,
                        b: 19,
                    }),
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: light(),
                    text: light(),
                    background: Some(AnsiColor::Rgb {
                        r: 55,
                        g: 65,
                        b: 81,
                    }),
                    text_bold: false,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 64,
                        g: 64,
                        b: 64,
                    }),
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: light(),
                    text: light(),
                    background: Some(AnsiColor::Rgb {
                        r: 55,
                        g: 65,
                        b: 81,
                    }),
                    text_bold: false,
                },
            ),
            (
                UsageFiveHour,
                ComponentColors {
                    icon: light(),
                    text: light(),
                    background: Some(AnsiColor::Rgb {
                        r: 45,
                        g: 50,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                UsageSevenDay,
                ComponentColors {
                    icon: light(),
                    text: light(),
                    background: Some(AnsiColor::Rgb {
                        r: 45,
                        g: 50,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 192,
                        b: 123,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 192,
                        b: 123,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 40,
                        g: 44,
                        b: 52,
                    }),
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 163,
                        g: 190,
                        b: 140,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 163,
                        g: 190,
                        b: 140,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 45,
                        g: 50,
                        b: 59,
                    }),
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 129,
                        g: 161,
                        b: 193,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 129,
                        g: 161,
                        b: 193,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 50,
                        g: 56,
                        b: 66,
                    }),
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 80,
                        g: 80,
                        b: 80,
                    }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
