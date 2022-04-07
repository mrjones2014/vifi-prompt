use ansi_term::{Color, Style};
use clap::{Args, Parser, Subcommand};
use std::{env, process::Command};

trait NoneIfEmpty {
    fn none_if_empty(&self) -> Option<String>;
}

impl NoneIfEmpty for Option<String> {
    fn none_if_empty(&self) -> Option<String> {
        if let Some(val) = self {
            if val.is_empty() {
                None
            } else {
                Some(val.to_string())
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Args)]
struct PromptArgs {
    #[clap(long)]
    status: i16,
    #[clap(long)]
    vi_mode: String,
}

#[derive(Debug, Args)]
struct InitArgs {}

#[derive(Debug, Subcommand)]
#[clap()]
enum Cmds {
    #[clap(about = "Print the Fish init script, to be piped through `source`")]
    Init(InitArgs),
    #[clap(about = "Print the prompt")]
    Prompt(PromptArgs),
}

#[derive(Debug, Parser)]
#[clap()]
struct VifiArgs {
    #[clap(subcommand)]
    cmd: Cmds,
}

enum ViMode {
    Normal,
    Insert,
    Replace,
    ReplaceOne,
    Visual,
}

struct GitStatus {
    ahead: bool,
    behind: bool,
    added: bool,
    modified: bool,
    deleted: bool,
    untracked: bool,
}

struct Context {
    work_dir: String,
    last_status: i16,
    git_branch: Option<String>,
    git_status: Option<GitStatus>,
    vi_mode: ViMode,
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
        }

        Some(status)
    } else {
        None
    }
}

impl Context {
    fn init(args: PromptArgs) -> Context {
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

fn main() {
    let args = VifiArgs::parse();
    match args.cmd {
        Cmds::Init(_) => println!("{}", include_str!("./init.fish")),
        Cmds::Prompt(args) => {
            let context = Context::init(args);
            let vi_symbol = match context.vi_mode {
                ViMode::Normal => "🅽 ",
                ViMode::Insert => "🅸 ",
                ViMode::Replace => "🆁 ",
                ViMode::ReplaceOne => "🆁 ",
                ViMode::Visual => "🆅 ",
            };

            let mut git_str = if let Some(branch) = context.git_branch {
                format!("  {}", branch)
            } else {
                "".into()
            };

            if let Some(status) = context.git_status {
                let mut symbols = "".into();

                if status.ahead {
                    symbols = format!("{}", symbols);
                }

                if status.behind {
                    symbols = format!("{}", symbols);
                }

                if status.modified {
                    symbols = format!("{}", symbols);
                }

                if status.added {
                    symbols = format!("{}", symbols);
                }

                if status.deleted {
                    symbols = format!("{}", symbols);
                }

                if status.untracked {
                    symbols = format!("{}", symbols);
                }

                if !symbols.is_empty() {
                    git_str = format!("{} [{}]", git_str, symbols);
                }
            }

            let mut prompt = format!(
                "\n {}{}",
                Color::Cyan.paint(context.work_dir),
                Color::RGB(247, 78, 39).paint(git_str)
            );

            if !prompt.ends_with('\n') {
                prompt = format!("{}\n", prompt);
            }

            let vi_symbol_color = match context.vi_mode {
                ViMode::Insert => {
                    if context.last_status == 0 {
                        Color::Cyan
                    } else {
                        Color::Red
                    }
                }
                ViMode::Normal => Color::RGB(67, 179, 115),
                ViMode::Replace | ViMode::ReplaceOne => Color::Purple,
                ViMode::Visual => Color::RGB(249, 249, 5),
            };

            prompt = format!("{} {} ", prompt, vi_symbol_color.paint(vi_symbol));
            print!("{}", Style::new().bold().paint(prompt));
        }
    }
}
