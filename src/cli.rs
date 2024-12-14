//! Command line parser

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Disable color output
    #[arg(short = 'C', long)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add new package or update to the running system
    Install {
        /// Path to the packages
        packages: Vec<String>,

        /// Installation prefix (path)
        #[arg(short, long, default_value_t = String::from("/"))]
        prefix: String,
    },

    /// Delete package(s) from the running system
    Remove {
        /// Package names `NAME`
        packages: Vec<String>,
    },

    /// Prints package metadata in human format
    Meta {
        /// Package name
        package: String,
    },

    /// Display a list of all installed packages
    List {
        #[arg(short, long)]
        version: bool,

        #[arg(short, long)]
        arch: bool,
    },
}
