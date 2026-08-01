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
    let black = || Some(AnsiColor::Rgb { r: 0, g: 0, b: 0 });

    ColorScheme::new(
        "Powerline Light",
        "Light powerline with backgrounds",
        vec![
            (
                Model,
                ComponentColors {
                    icon: black(),
                    text: black(),
                    background: Some(AnsiColor::Rgb {
                        r: 135,
                        g: 206,
                        b: 235,
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
                        r: 255,
                        g: 107,
                        b: 71,
                    }),
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 111,
                        g: 66,
                        b: 193,
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
                        r: 79,
                        g: 179,
                        b: 217,
                    }),
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 107,
                        g: 114,
                        b: 128,
                    }),
                    text_bold: false,
                },
            ),
            (
                Usage,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 40,
                        g: 167,
                        b: 69,
                    }),
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 255,
                        g: 193,
                        b: 7,
                    }),
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 40,
                        g: 167,
                        b: 69,
                    }),
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: white(),
                    text: white(),
                    background: Some(AnsiColor::Rgb {
                        r: 32,
                        g: 201,
                        b: 151,
                    }),
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 180,
                        g: 180,
                        b: 180,
                    }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
