use super::{ComponentIcons, IconSet};
use crate::config::types::ComponentId::*;

pub fn icon_set() -> IconSet {
    IconSet::new(
        "Minimal",
        "Simple ASCII/Unicode symbols",
        vec![
            (
                Model,
                ComponentIcons {
                    plain: "\u{273d}",
                    nerd_font: "\u{f2d0}",
                },
            ), // ✽
            (
                Directory,
                ComponentIcons {
                    plain: "\u{25d0}",
                    nerd_font: "\u{f024b}",
                },
            ), // ◐
            (
                Hostname,
                ComponentIcons {
                    plain: "@",
                    nerd_font: "\u{f108}",
                },
            ), // @
            (
                Git,
                ComponentIcons {
                    plain: "\u{203b}",
                    nerd_font: "\u{f02a2}",
                },
            ), // ※
            (
                ContextWindow,
                ComponentIcons {
                    plain: "\u{25d0}",
                    nerd_font: "\u{f49b}",
                },
            ), // ◐
            (
                Usage,
                ComponentIcons {
                    plain: "\u{1f4ca}",
                    nerd_font: "\u{f0a9e}",
                },
            ), // 📊
            (
                Cost,
                ComponentIcons {
                    plain: "\u{1f4b0}",
                    nerd_font: "\u{eec1}",
                },
            ), // 💰
            (
                Session,
                ComponentIcons {
                    plain: "\u{23f1}\u{fe0f}",
                    nerd_font: "\u{f19bb}",
                },
            ), // ⏱️
            (
                OutputStyle,
                ComponentIcons {
                    plain: "\u{1f3af}",
                    nerd_font: "\u{f12f5}",
                },
            ), // 🎯
            (
                Separator,
                ComponentIcons {
                    plain: " \u{2502} ",
                    nerd_font: " \u{2502} ",
                },
            ), // │
        ],
    )
}
