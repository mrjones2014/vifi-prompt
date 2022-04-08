use crate::context::{Context, ViMode};
use ansi_term::Color;
use clap::Parser;
use cli::{Cmds, VifiArgs};

mod cli;
mod context;
mod traits;

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

            let mut git_symbols = "".into();
            if let Some(status) = context.git_status {
                if status.ahead {
                    git_symbols = format!("{}⇡", git_symbols);
                }

                if status.behind {
                    git_symbols = format!("{}⇣", git_symbols);
                }

                if status.untracked {
                    git_symbols = format!("{}?", git_symbols);
                }

                if status.modified {
                    git_symbols = format!("{}!", git_symbols);
                }

                if status.added {
                    git_symbols = format!("{}+", git_symbols);
                }

                if status.deleted {
                    git_symbols = format!("{}✘", git_symbols);
                }
            }

            if !git_symbols.is_empty() {
                git_symbols = format!(" [{}]", git_symbols);
            }

            let mut prompt = format!(
                "\n {}{}{}",
                Color::Cyan.bold().paint(context.work_dir),
                Color::RGB(247, 78, 39).bold().paint(git_str),
                Color::RGB(247, 78, 39).paint(git_symbols)
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

            prompt = format!("{} {} ", prompt, vi_symbol_color.bold().paint(vi_symbol));
            print!("{}", prompt);
        }
    }
}
