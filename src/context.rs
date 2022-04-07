use crate::{cli::PromptArgs, traits::NoneIfEmpty};
use std::{env, process::Command};

pub enum ViMode {
    Normal,
    Insert,
    Replace,
    ReplaceOne,
    Visual,
}

pub struct GitStatus {
    pub ahead: bool,
    pub behind: bool,
    pub added: bool,
    pub modified: bool,
    pub deleted: bool,
    pub untracked: bool,
}

fn get_git_status() -> Option<GitStatus> {
    let stats = Command::new("git")
        .args(["status", "-sb"])
        .output()
        .map(|op| String::from_utf8(op.stdout).ok())
        .ok()
        .flatten()
        .map(|s| s.trim().to_string())
        .none_if_empty()
        .map(|s| {
            s.lines()
                .map(|line| line.trim().to_string())
                .collect::<Vec<String>>()
        });

    if let Some(stats) = stats {
        let mut status = GitStatus {
            ahead: false,
            behind: false,
            added: false,
            modified: false,
            deleted: false,
            untracked: false,
        };

        for stat in stats.iter() {
            if stat.starts_with('?') {
                status.untracked = true;
            }

            if stat.starts_with('M') {
                status.modified = true;
            }

            if stat.starts_with('D') {
                status.deleted = true;
            }

            if stat.starts_with('A') {
                status.added = true;
            }

            if stat.starts_with("##") && stat.contains('[') && stat.ends_with(']') {
                let ahead_behind_stats = &stat[stat.chars().position(|c| c == '[').unwrap()..];
                status.ahead = ahead_behind_stats.contains("ahead");
                status.behind = ahead_behind_stats.contains("behind");
            }

            // if we've got all the status indicators,
            // we don't need to keep checking
            if status.ahead
                && status.behind
                && status.added
                && status.modified
                && status.deleted
                && status.untracked
            {
                break;
            }
        }

        Some(status)
    } else {
        None
    }
}

pub struct Context {
    pub work_dir: String,
    pub last_status: i16,
    pub git_branch: Option<String>,
    pub git_status: Option<GitStatus>,
    pub vi_mode: ViMode,
}

impl Context {
    pub fn init(args: PromptArgs) -> Context {
        let last_status = args.status;

        let vi_mode = match args.vi_mode.as_str() {
            "normal" | "default" => ViMode::Normal,
            "insert" => ViMode::Insert,
            "replace" => ViMode::Replace,
            "replace_one" => ViMode::ReplaceOne,
            "visual" => ViMode::Visual,
            _ => ViMode::Insert,
        };

        let git_branch = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .map(|op| {
                String::from_utf8(op.stdout)
                    .ok()
                    .map(|s| s.trim().replace("\r\n", "").replace("\n", ""))
            })
            .ok()
            .flatten()
            .none_if_empty();

        let git_status = get_git_status();

        let current_dir = env::current_dir()
            .map(|p| {
                if git_branch.is_some() {
                    p.iter()
                        .last()
                        .map(|s| s.to_str())
                        .flatten()
                        .unwrap_or("[unknown]")
                        .to_string()
                } else {
                    p.to_str().unwrap_or("[unknown]").to_string()
                }
            })
            .unwrap_or_else(|_| "[unknown]".into());
        let home_dir = dirs_next::home_dir().map(|p| p.to_str().unwrap_or("[unknown]").to_string());
        let work_dir = home_dir
            .map(|home| current_dir.replacen(home.as_str(), "~", 1))
            .unwrap_or_else(|| "[unknown]".into());

        Context {
            work_dir,
            last_status,
            vi_mode,
            git_branch,
            git_status,
        }
    }
}
