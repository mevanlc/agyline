use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use ComponentId::*;

    // Based on "Later This Evening" by Gogh
    // https://github.com/Gogh-Co/Gogh
    ColorScheme::new(
        "Late",
        "Muted evening palette based on Later This Evening",
        vec![
            (
                Model,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 192,
                        g: 146,
                        b: 214,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 192,
                        g: 146,
                        b: 214,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 160,
                        g: 186,
                        b: 214,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 160,
                        g: 186,
                        b: 214,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 102,
                        g: 153,
                        b: 214,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 102,
                        g: 153,
                        b: 214,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 175,
                        g: 186,
                        b: 103,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 175,
                        g: 186,
                        b: 103,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 145,
                        g: 191,
                        b: 183,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 145,
                        g: 191,
                        b: 183,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                UsageFiveHour,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 210,
                        b: 137,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 210,
                        b: 137,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                UsageSevenDay,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 210,
                        b: 137,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 210,
                        b: 137,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 190,
                        b: 57,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 229,
                        g: 190,
                        b: 57,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 95,
                        g: 192,
                        b: 174,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 95,
                        g: 192,
                        b: 174,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 102,
                        g: 153,
                        b: 214,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 102,
                        g: 153,
                        b: 214,
                    }),
                    background: None,
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(AnsiColor::Color256 { c256: 130 }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
