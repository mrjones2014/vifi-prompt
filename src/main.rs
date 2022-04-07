use crate::context::{Context, ViMode};
use ansi_term::{Color, Style};
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
