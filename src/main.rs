mod lib;
mod renderer;
use std::{error::Error, sync::{mpsc, Mutex, Arc}, thread};

use renderer::Renderer;
// use lib::rm_api::Rickuest;
// use lib::query_language::*;
use clap::Parser;
use lib::rm_api::response::RMResponseEnum;

// #[derive(Parser)]
struct Args {
    mode: Mode,
}

enum Mode {
    Proxy,
    Direct,
}

#[derive(Debug)]
struct ResultState {
    value: Option<RMResponseEnum>,
    id: String,
}

#[derive(Default)]
pub struct AppState {
    results: Vec<ResultState>,
}



fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Mutex::new(AppState::default());
     
    let (tx, rx) = mpsc::channel::<String>();
    let renderer = Renderer::new(tx, app_state);

    let render_thread = thread::spawn(move || {
        renderer.start();
    });

    for request in rx {
        println!("Got request: {}", request);
    }

    render_thread.join().unwrap();

    Ok(())
}
