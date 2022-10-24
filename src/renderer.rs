use crossterm::{
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self},
    sync::{mpsc::Sender, Arc, Mutex},
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
        rm_api::{request::mock_request::MockRequest, response::RMResponseEnum},
    },
    AppState, ResultState,
};

enum InputMode {
    Normal,
    Editing,
}

pub struct Renderer {
    input: Input,
    input_mode: InputMode,
    app_state: Arc<Mutex<AppState>>,
    tx: Sender<String>,
}

impl Renderer {
    pub fn new(tx: Sender<String>, app_state: Arc<Mutex<AppState>>) -> Self {
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
                        tx.send(renderer.input.value().to_string()).unwrap();
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

    let msg = vec![
        Span::raw("Press "),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" then "),
        Span::styled(" q ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("to quit."),
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

    renderer
        .app_state
        .lock()
        .unwrap()
        .results
        .values()
        .for_each(|result_state|{
            f.render_widget(create_response_enum_widget(result_state), chunks[2]);
        });

   
}

fn create_response_enum_widget(result_state: &ResultState) -> List {
    let response = result_state.value.clone();

    match response {
        None => List::new(vec![ListItem::new(vec![Spans::from(Span::raw("None"))])]),
        Some(RMResponseEnum::Characters(characters_page)) => {
            let characters: Vec<ListItem> = characters_page
                .results
                .iter()
                .map(|character| {
                    let name_span =
                        Span::styled(character.name.clone(), Style::default().fg(Color::Green));
                    let id_span = Span::styled(
                        format!("({}) ", character.id),
                        Style::default().fg(Color::Yellow),
                    );
                    let status_span = match character.status.as_str() {
                        "Alive" => Span::styled(
                            format!(" [{}] ", character.status),
                            Style::default().fg(Color::Green),
                        ),
                        "Dead" => Span::styled(
                            format!(" [{}] ", character.status),
                            Style::default().fg(Color::Red),
                        ),
                        _ => Span::styled(
                            format!(" [{}] ", character.status),
                            Style::default().fg(Color::Yellow),
                        ),
                      
                    };
                    ListItem::new(vec![Spans::from(vec![id_span, name_span, status_span])])
                })
                .collect();

            List::new(characters).block(Block::default().borders(Borders::ALL).title("Characters"))
        }
        Some(RMResponseEnum::Episodes(episodes_page)) => {
            let episodes: Vec<ListItem> = episodes_page
                .results
                .iter()
                .map(|episode| {
                    let name_span =
                        Span::styled(episode.name.clone(), Style::default().fg(Color::Green));
                    let id_span = Span::styled(
                        format!("({})", episode.id),
                        Style::default().fg(Color::Yellow),
                    );
                    ListItem::new(vec![Spans::from(vec![name_span, id_span])])
                })
                .collect();

            List::new(episodes).block(Block::default().borders(Borders::ALL).title("Episodes"))
        }
        Some(RMResponseEnum::Locations(locations_page)) => {
            let locations: Vec<ListItem> = locations_page
                .results
                .iter()
                .map(|location| {
                    let name_span =
                        Span::styled(location.name.clone(), Style::default().fg(Color::Green));
                    let id_span = Span::styled(
                        format!("({})", location.id),
                        Style::default().fg(Color::Yellow),
                    );
                    ListItem::new(vec![Spans::from(vec![name_span, id_span])])
                })
                .collect();

            List::new(locations).block(Block::default().borders(Borders::ALL).title("Locations"))
        }
    }
}
