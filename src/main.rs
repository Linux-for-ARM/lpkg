use std::process::exit;

use clap::Parser;
use lpkg::cli::*;
use lpkg::err_msg;

use lpkg::install::install;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Install { packages, prefix } => {
            if let Err(why) = install(&packages, &prefix) {
                err_msg!("Install error: {why}");
                exit(1);
            }
        }
        _ => {}
    }
}
