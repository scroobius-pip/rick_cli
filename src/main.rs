mod lib;
mod renderer;
// use futures::executor::block_on;
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
use lib::rm_api::{request::direct_request::DirectRequest, response::RMResponseEnum};

use crate::lib::query_api;

// #[derive(Parser)]
struct Args {
    mode: Mode,
}

enum Mode {
    Proxy,
    Direct,
}

#[derive(Debug,Clone)]
struct ResultState {
    value: Option<RMResponseEnum>,
    id: String,
    error_msg: Option<String>,
}

#[derive(Default)]
pub struct AppState {
    results: HashMap<String, ResultState>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Arc::new(Mutex::new(AppState::default()));
    let render_app_state = app_state.clone();
    let (tx, rx) = mpsc::channel::<String>();

    let render_thread = thread::spawn(move || {
        let renderer = Renderer::new(tx, render_app_state);
        renderer.start();
    });

    for request in rx {
        apply_request_state(request, app_state.clone()).await;
        // use tokio spawn for the above
        // let app_state = app_state.clone();
        // tokio::spawn(async move {
        //     let result = apply_request_state(request, app_state).await;
          
        // });
    }
    render_thread.join().unwrap();
    Ok(())
}

async fn apply_request_state(request: String, app_state: Arc<Mutex<AppState>>) {
    let request_str = request.as_str();
    let mut state = app_state.lock().unwrap();
    state.results.clear();
    state.results.insert(
        request_str.to_string(),
        ResultState {
            id: request_str.to_string(),
            value: None,
            error_msg: None,
        },
    );
    // let mock_query_result = query_api(MockRequest, request_str).await;
    let query_result = query_api(DirectRequest, request_str).await;
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
