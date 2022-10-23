mod lib;
mod renderer;
use futures::executor::block_on;
use renderer::Renderer;
use std::{
    collections::HashMap,
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread,
};
// use lib::rm_api::Rickuest;
// use lib::query_language::*;
use clap::Parser;
use lib::rm_api::response::RMResponseEnum;
use rocket::{futures, tokio};

use crate::lib::mock_query_api;

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
    error_msg: Option<String>,
}

#[derive(Default)]
pub struct AppState {
    results: HashMap<String, ResultState>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let render_app_state = app_state.clone();
    let (tx, rx) = mpsc::channel::<String>();
    let render_thread = thread::spawn(move || {
        let renderer = Renderer::new(tx, render_app_state);
        renderer.start();
    });

    for request in rx {
        let app_state = app_state.clone();
        thread::spawn(move || {
            block_on(apply_request_state(request, app_state));
        });
    }
    render_thread.join().unwrap();
    Ok(())
}

async fn apply_request_state(request: String, app_state: Arc<Mutex<AppState>>) {
    let request_str = request.as_str();
    let mut state = app_state.lock().unwrap();
    state.results.insert(
        request_str.to_string(),
        ResultState {
            id: request_str.to_string(),
            value: None,
            error_msg: None,
        },
    );
    let query_result = mock_query_api(request_str).await;
    let new_result_state = match query_result {
        Ok(response) => ResultState {
            error_msg: None,
            id: request_str.to_string(),
            value: Some(response),
        },
        Err(err) => ResultState {
            error_msg: Some(err.to_string()),
            id: request_str.to_string(),
            value: None,
        },
    };
    state.results.insert(request, new_result_state);
}
