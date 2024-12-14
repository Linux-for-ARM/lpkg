// use clap::Parser;
// use lpkg::cli::*;

use std::env::args;
use std::fs::File;
use std::path::Path;
use lpkg::archive::Archive;

use lpkg::{msg, ok_msg};

fn main() {
    // let _cli = Cli::parse();
    let argv = args().collect::<Vec<_>>();
    let f = File::open(&argv[1]).unwrap();
    let mut a = Archive::new(&Path::new(&argv[0]), f);
    a.ls().unwrap();
    a.ls().unwrap();
    msg!("Reading metadata...");
    ok_msg!("{:#?}", a.metadata().unwrap());
}
