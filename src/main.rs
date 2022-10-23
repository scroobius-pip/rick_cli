mod app;
mod lib;
use std::error::Error;

use app::init_app;
// use lib::rm_api::Rickuest;
// use lib::query_language::*;
use clap::Parser;

// #[derive(Parser)]
struct Args {
    mode: Mode,
}

enum Mode {
    Proxy,
    Direct,
}

fn main() -> Result<(), Box<dyn Error>> {
    //    RICK AND MORTY API
    // TYPE A QUERY TO REQUEST DATA FROM THE RICK AND MORTY API
    // SYNTAX: CHARACTER::Name(rick)::Page(1)::Contains(rick,name)::Length(10, episode)::Index(0)::Sort(ASC, name)

    // let args = Args::parse();

    // if args.proxy {
    //     println!("Proxy mode enabled");
    //

    // setup terminal
    init_app()?;
    Ok(())
}
