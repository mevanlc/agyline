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
