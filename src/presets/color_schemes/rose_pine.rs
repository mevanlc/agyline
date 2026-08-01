use super::{ColorScheme, ComponentColors};
use crate::config::types::{AnsiColor, ComponentId};

pub fn scheme() -> ColorScheme {
    use ComponentId::*;

    ColorScheme::new(
        "Rose Pine",
        "Soft dark palette with backgrounds",
        vec![
            (
                Model,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 235,
                        g: 188,
                        b: 186,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 235,
                        g: 188,
                        b: 186,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 25,
                        g: 23,
                        b: 36,
                    }),
                    text_bold: false,
                },
            ),
            (
                Directory,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 196,
                        g: 167,
                        b: 231,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 196,
                        g: 167,
                        b: 231,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 38,
                        g: 35,
                        b: 58,
                    }),
                    text_bold: false,
                },
            ),
            (
                Hostname,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 156,
                        g: 207,
                        b: 216,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 156,
                        g: 207,
                        b: 216,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 38,
                        g: 35,
                        b: 58,
                    }),
                    text_bold: false,
                },
            ),
            (
                Git,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 156,
                        g: 207,
                        b: 216,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 156,
                        g: 207,
                        b: 216,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 31,
                        g: 29,
                        b: 46,
                    }),
                    text_bold: false,
                },
            ),
            (
                ContextWindow,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 222,
                        b: 244,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 224,
                        g: 222,
                        b: 244,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 82,
                        g: 79,
                        b: 103,
                    }),
                    text_bold: false,
                },
            ),
            (
                Usage,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 246,
                        g: 193,
                        b: 119,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 246,
                        g: 193,
                        b: 119,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 35,
                        g: 33,
                        b: 54,
                    }),
                    text_bold: false,
                },
            ),
            (
                Cost,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 246,
                        g: 193,
                        b: 119,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 246,
                        g: 193,
                        b: 119,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 35,
                        g: 33,
                        b: 54,
                    }),
                    text_bold: false,
                },
            ),
            (
                Session,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 156,
                        g: 207,
                        b: 216,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 156,
                        g: 207,
                        b: 216,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 42,
                        g: 39,
                        b: 63,
                    }),
                    text_bold: false,
                },
            ),
            (
                OutputStyle,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 49,
                        g: 116,
                        b: 143,
                    }),
                    text: Some(AnsiColor::Rgb {
                        r: 49,
                        g: 116,
                        b: 143,
                    }),
                    background: Some(AnsiColor::Rgb {
                        r: 38,
                        g: 35,
                        b: 58,
                    }),
                    text_bold: false,
                },
            ),
            (
                Separator,
                ComponentColors {
                    icon: Some(AnsiColor::Rgb {
                        r: 110,
                        g: 106,
                        b: 134,
                    }),
                    text: None,
                    background: None,
                    text_bold: false,
                },
            ),
        ],
    )
}
