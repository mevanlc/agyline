use super::{ComponentIcons, IconSet};
use crate::config::types::ComponentId::*;

pub fn icon_set() -> IconSet {
    IconSet::new(
        "Nerd Font",
        "Standard Nerd Font icons",
        vec![
            (
                Model,
                ComponentIcons {
                    plain: "\u{1f916}",
                    nerd_font: "\u{e26d}",
                },
            ), // 🤖 /
            (
                Directory,
                ComponentIcons {
                    plain: "\u{1f4c1}",
                    nerd_font: "\u{f024b}",
                },
            ), // 📁 / 󰉋
            (
                Hostname,
                ComponentIcons {
                    plain: "\u{1f5a5}\u{fe0f}",
                    nerd_font: "\u{f108}",
                },
            ), // 🖥️ / 
            (
                Git,
                ComponentIcons {
                    plain: "\u{1f33f}",
                    nerd_font: "\u{f02a2}",
                },
            ), // 🌿 / 󰊢
            (
                ContextWindow,
                ComponentIcons {
                    plain: "\u{26a1}\u{fe0f}",
                    nerd_font: "\u{f49b}",
                },
            ), // ⚡️ /
            (
                UsageFiveHour,
                ComponentIcons {
                    plain: "\u{1f4ca}",
                    nerd_font: "\u{f0a9e}",
                },
            ), // 📊 / 󰪞
            (
                UsageSevenDay,
                ComponentIcons {
                    plain: "\u{1f4ca}",
                    nerd_font: "\u{f0a9e}",
                },
            ), // 📊 / 󰪞
            (
                Cost,
                ComponentIcons {
                    plain: "\u{1f4b0}",
                    nerd_font: "\u{eec1}",
                },
            ), // 💰 /
            (
                Session,
                ComponentIcons {
                    plain: "\u{23f1}\u{fe0f}",
                    nerd_font: "\u{f19bb}",
                },
            ), // ⏱️ / 󱦻
            (
                OutputStyle,
                ComponentIcons {
                    plain: "\u{1f3af}",
                    nerd_font: "\u{f12f5}",
                },
            ), // 🎯 / 󱋵
            (
                Separator,
                ComponentIcons {
                    plain: " | ",
                    nerd_font: " | ",
                },
            ),
        ],
    )
}
