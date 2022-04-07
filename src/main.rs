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

struct Context {
    work_dir: String,
    last_status: i16,
    git_branch: Option<String>,
    vi_mode: ViMode,
}

impl Context {
    fn init(args: PromptArgs) -> Context {
        let current_dir = env::current_dir()
            .map(|p| p.to_str().unwrap_or("[unknown]").to_string())
            .unwrap_or_else(|_| "[unknown]".into());
        let home_dir = dirs_next::home_dir().map(|p| p.to_str().unwrap_or("[unknown]").to_string());
        let work_dir = home_dir
            .map(|home| current_dir.replacen(home.as_str(), "~", 1))
            .unwrap_or_else(|| "[unknown]".into());

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

        Context {
            work_dir,
            last_status: args.status,
            vi_mode,
            git_branch,
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

            let git_str = if let Some(branch) = context.git_branch {
                format!("  {}", branch)
            } else {
                "".into()
            };

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
