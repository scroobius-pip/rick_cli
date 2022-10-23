use crossterm::{
    cursor::{Hide, Show},
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    fmt::Display,
    io::{self, stdout, Write},
    sync::{mpsc::Sender, Mutex},
    thread,
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use tui_input::{
    backend::{crossterm as backend, crossterm::EventHandler},
    Input,
};

use crate::{
    lib::{
        query_language::operation_list::OperationList,
        query_language::operation_list::OperationListEvaluator,
        rm_api::{request::MockRequest, response::RMResponseEnum},
    },
    AppState,
};

enum InputMode {
    Normal,
    Editing,
}


pub struct Renderer {
    input: Input,
    input_mode: InputMode,
    app_state: Mutex<AppState>,
    tx: Sender<String>,
}

#[derive(Debug)]
struct ResultState {
    value: Option<RMResponseEnum>,
    id: String,
}

impl Renderer {
    pub fn new(tx: Sender<String>, app_state: Mutex<AppState>) -> Self {
        Self {
            input_mode: InputMode::Editing,
            input: Input::default(),
            app_state,
            tx,
        }
    }

    pub fn start(self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let res = run_app(&mut terminal, self);
        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }
}

// pub fn init_app(tx: Sender<String>) -> Result<(), Box<dyn Error>> {

// }

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut renderer: Renderer) -> io::Result<()> {
    let tx = renderer.tx.clone();
    loop {
        terminal.draw(|f| ui(f, &mut renderer))?;

        if let Event::Key(key) = event::read()? {
            match renderer.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        renderer.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        // app.results.push(app.input.value().into());
                        // let new_query = query_api(&app.input.value());
                        tx.send(renderer.input.value().to_string()).unwrap();
                        // add new_query to thread pool
                        // let new_query = thread::spawn(move || {
                        //     let new_query = new_query.await;
                        //     new_query.unwrap()
                        // });
                        // app.input.reset();
                    }
                    KeyCode::Esc => {
                        renderer.input_mode = InputMode::Normal;
                    }
                    _ => {
                        renderer.input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, renderer: &Renderer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // let (msg, style) = match app.input_mode {
    //     InputMode::Normal => (
    //         vec![
    //             Span::raw("Press "),
    //             Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
    //             Span::raw(" to exit, "),
    //             Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
    //             Span::raw(" to start editing."),
    //         ],
    //         Style::default().add_modifier(Modifier::RAPID_BLINK),
    //     ),
    //     InputMode::Editing => (
    //        ,
    //         Style::default(),
    //     ),
    // };

    let msg = vec![
        Span::raw("Press "),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to stop editing, "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to send query"),
    ];
    let style = Style::default();

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = (renderer.input.cursor() as u16).max(width) - width;
    let input = Paragraph::new(renderer.input.value())
        .style(Style::default().fg(Color::Yellow))
        .scroll((0, scroll))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[1].x + (renderer.input.cursor() as u16).min(width) + 1,
        // Move one line down, from the border to the input line
        chunks[1].y + 1,
    );

    let messages: Vec<ListItem> = renderer
        .app_state
        .lock()
        .unwrap()
        .results
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {:?}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[2]);
}
