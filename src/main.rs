use crate::context::{Context, ViMode};
use ansi_term::Color;
use clap::Parser;
use cli::{Cmds, VifiArgs};
use std::{env, time::Duration};

mod cli;
mod context;
mod traits;

fn main() {
    let args = VifiArgs::parse();
    match args.cmd {
        Cmds::Init(_) => println!("{}", include_str!("./init.fish")),
        Cmds::RightPrompt(args) => {
            let last_cmd = args.last_command.trim().to_lowercase();
            let last_cmd_is_editor = env::var("EDITOR")
                .map(|editor| last_cmd.starts_with(&editor.to_lowercase()))
                .unwrap_or(false);
            // only show time if greater than 3s
            // or if last command was not a text editor
            if args.last_duration < 2999
                || last_cmd.starts_with("vim")
                || last_cmd.starts_with("nvim")
                || last_cmd.starts_with("hx")
                || last_cmd.starts_with("emacs")
                || last_cmd_is_editor
            {
                return;
            }

            let duration = Duration::from_millis(args.last_duration).as_secs();
            let (h, s) = (duration / 3600, duration % 3600);
            let (m, s) = (s / 60, s % 60);

            let mut units: Vec<String> = vec![];
            (h > 0).then(|| units.push(format!("{}h", h)));
            (m > 0).then(|| units.push(format!("{}m", m)));
            (s > 0).then(|| units.push(format!("{}s", s)));

            let mut duration_str = units.join(" ");

            if !duration_str.is_empty() {
                duration_str = format!(" {}", duration_str);
                println!("{}", Color::RGB(88, 96, 104).paint(duration_str));
            }
        }
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
