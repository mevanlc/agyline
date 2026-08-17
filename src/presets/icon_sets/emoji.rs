use super::{ComponentIcons, IconSet};
use crate::config::types::ComponentId::*;

pub fn icon_set() -> IconSet {
    IconSet::new(
        "Emoji",
        "Standard emoji icons",
        vec![
            (
                AgentState,
                ComponentIcons {
                    plain: "\u{25cf}",
                    nerd_font: "\u{f169d}",
                },
            ), // ● / 󱚝
            (
                Model,
                ComponentIcons {
                    plain: "\u{1f916}",
                    nerd_font: "\u{f09d1}",
                },
            ), // 🤖 / 󰧑
            (
                Directory,
                ComponentIcons {
                    plain: "\u{1f4c1}",
                    nerd_font: "\u{f024b}",
                },
            ), // 📁
            (
                TaskCount,
                ComponentIcons {
                    plain: "\u{2699}\u{fe0f}",
                    nerd_font: "\u{f048b}",
                },
            ), // ⚙️ / 󰒋
            (
                ExecutionMode,
                ComponentIcons {
                    plain: "\u{1f9ed}",
                    nerd_font: "\u{f0633}",
                },
            ), // 🧭 / 󰘳
            (
                VimMode,
                ComponentIcons {
                    plain: "\u{2328}\u{fe0f}",
                    nerd_font: "\u{e7c5}",
                },
            ), // ⌨️ / 
            (
                ArtifactCount,
                ComponentIcons {
                    plain: "\u{1f4e6}",
                    nerd_font: "\u{f03d6}",
                },
            ), // 📦 / 󰏖
            (
                PendingInput,
                ComponentIcons {
                    plain: "\u{1f4ac}",
                    nerd_font: "\u{f0b79}",
                },
            ), // 💬 / 󰭹
            (
                ToolConfirmation,
                ComponentIcons {
                    plain: "\u{1f514}",
                    nerd_font: "\u{f009a}",
                },
            ), // 🔔 / 󰂚
            (
                Sandbox,
                ComponentIcons {
                    plain: "\u{1f6e1}\u{fe0f}",
                    nerd_font: "\u{f0208}",
                },
            ), // 🛡️ / 󰈈
            (
                PlanTier,
                ComponentIcons {
                    plain: "\u{2b50}",
                    nerd_font: "\u{f04ce}",
                },
            ), // ⭐ / 󰓎
            (
                Email,
                ComponentIcons {
                    plain: "\u{1f464}",
                    nerd_font: "\u{f0009}",
                },
            ), // 👤 / 󰀉
            (
                Worktree,
                ComponentIcons {
                    plain: "\u{1f333}",
                    nerd_font: "\u{f1bb}",
                },
            ), // 🌳
            (
                Hostname,
                ComponentIcons {
                    plain: "\u{1f5a5}\u{fe0f}",
                    nerd_font: "\u{f108}",
                },
            ), // 🖥️
            (
                Git,
                ComponentIcons {
                    plain: "\u{1f33f}",
                    nerd_font: "\u{f02a2}",
                },
            ), // 🌿
            (
                PullRequest,
                ComponentIcons {
                    plain: "\u{1f500}",
                    nerd_font: "\u{ea64}",
                },
            ), // 🔀
            (
                ContextWindow,
                ComponentIcons {
                    plain: "\u{26a1}\u{fe0f}",
                    nerd_font: "\u{f49b}",
                },
            ), // ⚡️
            (
                UsageFiveHour,
                ComponentIcons {
                    plain: "\u{1f4ca}",
                    nerd_font: "\u{f0a9e}",
                },
            ), // 📊
            (
                UsageSevenDay,
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
                    plain: " | ",
                    nerd_font: " | ",
                },
            ),
        ],
    )
}
