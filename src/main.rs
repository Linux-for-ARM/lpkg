use std::process::exit;

use clap::Parser;
use lpkg::cli::*;
use lpkg::err_msg;

use lpkg::install::install;
use lpkg::info::{list_all, info};
use lpkg::log::print_experimental_banner;

fn main() {
    let cli = Cli::parse();
    print_experimental_banner();

    match cli.command {
        Command::Install { packages, prefix } => {
            if let Err(why) = install(&packages, &prefix) {
                err_msg!("Install error: {why}");
                exit(1);
            }
        }
        Command::List { version, arch } => {
            if let Err(why) = list_all(version, arch) {
                err_msg!("Database error: {why}");
                exit(1);
            }
        }
        Command::Meta { package } => {
            if let Err(why) = info(&package) {
                err_msg!("Database error: {why}");
                exit(1);
            }
        }
        _ => {}
    }
}
