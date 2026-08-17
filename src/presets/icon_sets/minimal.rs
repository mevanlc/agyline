use super::{ComponentIcons, IconSet};
use crate::config::types::ComponentId::*;

pub fn icon_set() -> IconSet {
    IconSet::new(
        "Minimal",
        "Simple ASCII/Unicode symbols",
        vec![
            (
                AgentState,
                ComponentIcons {
                    plain: "\u{25cf}",
                    nerd_font: "\u{25cf}",
                },
            ), // ●
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
                TaskCount,
                ComponentIcons {
                    plain: "#",
                    nerd_font: "#",
                },
            ),
            (
                ExecutionMode,
                ComponentIcons {
                    plain: "M",
                    nerd_font: "M",
                },
            ),
            (
                VimMode,
                ComponentIcons {
                    plain: "V",
                    nerd_font: "V",
                },
            ),
            (
                ArtifactCount,
                ComponentIcons {
                    plain: "A",
                    nerd_font: "A",
                },
            ),
            (
                PendingInput,
                ComponentIcons {
                    plain: "?",
                    nerd_font: "?",
                },
            ),
            (
                ToolConfirmation,
                ComponentIcons {
                    plain: "!",
                    nerd_font: "!",
                },
            ),
            (
                Sandbox,
                ComponentIcons {
                    plain: "S",
                    nerd_font: "S",
                },
            ),
            (
                PlanTier,
                ComponentIcons {
                    plain: "*",
                    nerd_font: "*",
                },
            ),
            (
                Email,
                ComponentIcons {
                    plain: "@",
                    nerd_font: "@",
                },
            ),
            (
                Worktree,
                ComponentIcons {
                    plain: "W",
                    nerd_font: "\u{f1bb}",
                },
            ),
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
                PullRequest,
                ComponentIcons {
                    plain: "PR",
                    nerd_font: "\u{ea64}",
                },
            ),
            (
                ContextWindow,
                ComponentIcons {
                    plain: "\u{25d0}",
                    nerd_font: "\u{f49b}",
                },
            ), // ◐
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
                    plain: " \u{2502} ",
                    nerd_font: " \u{2502} ",
                },
            ), // │
        ],
    )
}
