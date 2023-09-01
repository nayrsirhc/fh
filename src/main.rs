mod cli;

use std::io::IsTerminal;

use clap::Parser;

use crate::cli::{
    cmd::{CommandExecute, FhSubcommands::*},
    Cli,
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> color_eyre::Result<std::process::ExitCode> {
    color_eyre::config::HookBuilder::default()
        .theme(if !std::io::stderr().is_terminal() {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::dark()
        })
        .install()?;

    let cli = Cli::parse();
    cli.instrumentation.setup().await?;

    match cli.subcommand {
        Add(add) => add.execute().await,
        Init(init) => init.execute().await,
        List(list) => list.execute().await,
        Search(search) => search.execute().await,
    }
}
