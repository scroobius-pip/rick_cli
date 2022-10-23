mod app;
mod lib;
use std::{error::Error, sync::mpsc, thread};

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
    let (send_requests, rx) = mpsc::channel::<String>();
    let render_thread = thread::spawn(move || {
        init_app(send_requests).unwrap();
    });

    for request in rx {
        // println!("{}", request);
    }


    render_thread.join().unwrap();

    Ok(())
}
