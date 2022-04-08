use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args)]
pub struct PromptArgs {
    #[clap(long)]
    pub status: i16,
    #[clap(long)]
    pub vi_mode: String,
}

#[derive(Debug, Args)]
pub struct RightPromptArgs {
    #[clap(long)]
    pub last_duration: u64,
    #[clap(long)]
    pub last_command: String,
}

#[derive(Debug, Args)]
pub struct InitArgs {}

#[derive(Debug, Subcommand)]
#[clap()]
pub enum Cmds {
    #[clap(about = "Print the Fish init script, to be piped through `source`")]
    Init(InitArgs),
    #[clap(about = "Print the prompt")]
    Prompt(PromptArgs),
    #[clap(about = "Print the right side prompt")]
    RightPrompt(RightPromptArgs),
}

#[derive(Debug, Parser)]
#[clap()]
pub struct VifiArgs {
    #[clap(subcommand)]
    pub cmd: Cmds,
}
