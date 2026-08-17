use super::{ComponentIcons, IconSet};
use crate::config::types::ComponentId::*;

pub fn icon_set() -> IconSet {
    IconSet::new(
        "Late",
        "Evening-themed icons for late-night sessions",
        vec![
            (
                AgentState,
                ComponentIcons {
                    plain: "\u{1f31f}",
                    nerd_font: "\u{f169d}",
                },
            ), // 🌟 / 󱚝
            (
                Model,
                ComponentIcons {
                    plain: "\u{1f319}",
                    nerd_font: "\u{f03d2}",
                },
            ), // 🌙 / 󰏒 md-owl
            (
                Directory,
                ComponentIcons {
                    plain: "\u{1f4c2}",
                    nerd_font: "\u{f069d}",
                },
            ), // 📂 / 󰚝 md-folder_star
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
                    plain: "\u{1f332}",
                    nerd_font: "\u{f1bb}",
                },
            ), // 🌲 /  fa-tree
            (
                Hostname,
                ComponentIcons {
                    plain: "\u{1f5a5}\u{fe0f}",
                    nerd_font: "\u{f108}",
                },
            ), // 🖥️ /  fa-desktop
            (
                Git,
                ComponentIcons {
                    plain: "\u{1f500}",
                    nerd_font: "\u{f062c}",
                },
            ), // 🔀 / 󰘬 md-source_branch
            (
                PullRequest,
                ComponentIcons {
                    plain: "\u{1f500}",
                    nerd_font: "\u{ea64}",
                },
            ), // 🔀 /  cod-git_pull_request
            (
                ContextWindow,
                ComponentIcons {
                    plain: "\u{1f52e}",
                    nerd_font: "\u{f0996}",
                },
            ), // 🔮 / 󰦖 md-progress_clock
            (
                UsageFiveHour,
                ComponentIcons {
                    plain: "\u{1f4c8}",
                    nerd_font: "\u{f0430}",
                },
            ), // 📈 / 󰐰 md-pulse
            (
                UsageSevenDay,
                ComponentIcons {
                    plain: "\u{1f4c8}",
                    nerd_font: "\u{f0430}",
                },
            ), // 📈 / 󰐰 md-pulse
            (
                Cost,
                ComponentIcons {
                    plain: "\u{1fa99}",
                    nerd_font: "\u{f188f}",
                },
            ), // 🪙 / 󱢏 md-hand_coin
            (
                Session,
                ComponentIcons {
                    plain: "\u{1f551}",
                    nerd_font: "\u{f051f}",
                },
            ), // 🕑 / 󰔟 md-timer_sand
            (
                OutputStyle,
                ComponentIcons {
                    plain: "\u{1f4ac}",
                    nerd_font: "\u{f0f2d}",
                },
            ), // 💬 / 󰼭 md-typewriter
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
