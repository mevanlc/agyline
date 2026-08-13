use super::{Component, ComponentData};
use crate::config::types::ComponentId;
use crate::core::input::InputData;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, PartialEq)]
enum GitStatus {
    Clean,
    Dirty,
    Conflicts,
}

pub struct GitComponent {
    show_sha: bool,
}

impl Default for GitComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl GitComponent {
    pub fn new() -> Self {
        Self { show_sha: false }
    }

    pub fn with_sha(mut self, show_sha: bool) -> Self {
        self.show_sha = show_sha;
        self
    }

    fn is_git_repository(dir: &str) -> bool {
        Command::new("git")
            .args(["--no-optional-locks", "rev-parse", "--git-dir"])
            .current_dir(dir)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn get_branch(dir: &str) -> Option<String> {
        let try_cmd = |args: &[&str]| -> Option<String> {
            let output = Command::new("git")
                .args(args)
                .current_dir(dir)
                .output()
                .ok()?;
            if output.status.success() {
                let s = String::from_utf8(output.stdout).ok()?.trim().to_string();
                if !s.is_empty() { Some(s) } else { None }
            } else {
                None
            }
        };

        try_cmd(&["--no-optional-locks", "branch", "--show-current"])
            .or_else(|| try_cmd(&["--no-optional-locks", "symbolic-ref", "--short", "HEAD"]))
    }

    pub(crate) fn branch_name(dir: &str) -> Option<String> {
        if !Self::is_git_repository(dir) {
            return None;
        }

        Some(Self::get_branch(dir).unwrap_or_else(|| "detached".into()))
    }

    fn get_status(dir: &str) -> GitStatus {
        let output = Command::new("git")
            .args(["--no-optional-locks", "status", "--porcelain"])
            .current_dir(dir)
            .output();

        match output {
            Ok(o) if o.status.success() => {
                let text = String::from_utf8(o.stdout).unwrap_or_default();
                if text.trim().is_empty() {
                    GitStatus::Clean
                } else if text.contains("UU") || text.contains("AA") || text.contains("DD") {
                    GitStatus::Conflicts
                } else {
                    GitStatus::Dirty
                }
            }
            _ => GitStatus::Clean,
        }
    }

    fn get_commit_count(dir: &str, range: &str) -> u32 {
        Command::new("git")
            .args(["--no-optional-locks", "rev-list", "--count", range])
            .current_dir(dir)
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    String::from_utf8(o.stdout).ok()?.trim().parse().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }

    fn get_sha(dir: &str) -> Option<String> {
        let output = Command::new("git")
            .args(["--no-optional-locks", "rev-parse", "--short=7", "HEAD"])
            .current_dir(dir)
            .output()
            .ok()?;

        if output.status.success() {
            let sha = String::from_utf8(output.stdout).ok()?.trim().to_string();
            if sha.is_empty() { None } else { Some(sha) }
        } else {
            None
        }
    }
}

impl Component for GitComponent {
    fn collect(&self, input: &InputData) -> Option<ComponentData> {
        let dir = &input.workspace.current_dir;
        let branch = Self::branch_name(dir)?;
        let status = Self::get_status(dir);
        let ahead = Self::get_commit_count(dir, "@{u}..HEAD");
        let behind = Self::get_commit_count(dir, "HEAD..@{u}");
        let sha = if self.show_sha {
            Self::get_sha(dir)
        } else {
            None
        };

        let mut metadata = HashMap::new();
        metadata.insert("branch".into(), branch.clone());
        metadata.insert("status".into(), format!("{:?}", status));
        metadata.insert("ahead".into(), ahead.to_string());
        metadata.insert("behind".into(), behind.to_string());
        if let Some(ref s) = sha {
            metadata.insert("sha".into(), s.clone());
        }

        let mut status_parts = Vec::new();
        match status {
            GitStatus::Clean => status_parts.push("\u{2713}".into()),
            GitStatus::Dirty => status_parts.push("\u{25cf}".into()),
            GitStatus::Conflicts => status_parts.push("\u{26a0}".into()),
        }
        if ahead > 0 {
            status_parts.push(format!("\u{2191}{}", ahead));
        }
        if behind > 0 {
            status_parts.push(format!("\u{2193}{}", behind));
        }
        if let Some(ref s) = sha {
            status_parts.push(s.clone());
        }

        Some(ComponentData {
            primary: branch,
            secondary: status_parts.join(" "),
            metadata,
        })
    }

    fn id(&self) -> ComponentId {
        ComponentId::Git
    }
}
