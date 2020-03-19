use sse_client::EventSource;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{error::Error, io, thread, time};
use termion::{async_stdin, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, SelectableList, Text},
    Terminal,
};

//use std::collections::HashMap;

//struct App<'a> {
struct App {
    //items: Vec<&'a str>,
    items: Vec<String>,
    selected: usize,
}

//impl<'a> App<'a> {
//fn new() -> App<'a> {
impl App {
    fn new() -> App {
        App {
            items: vec![
                String::from("hi"),
                String::from("hir1"),
                String::from("hi3"),
                String::from("hi5"),
            ],
            selected: 0,
        }
    }
}

fn draw(app: Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());

            let style = Style::default();
            let locked_app = app.lock().unwrap();
            let mut items = SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Waitqueue"))
                .items(&locked_app.items)
                .style(style)
                .select(Some(locked_app.selected))
                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                .highlight_symbol(">");
            f.render(&mut items, chunks[1]);

            let text = [Text::raw("idk this is where info will be")];
            let mut info =
                Paragraph::new(text.iter()).block(Block::default().borders(Borders::ALL).title("Info"));
            //.start_corner(Corner::BottomLeft);
            f.render(&mut info, chunks[0]);
        })?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = async_stdin().bytes();
    
    // App
    let app = Arc::new(Mutex::new(App::new()));

    let evt_src = EventSource::new("https://oh.zvs.io/sse").unwrap();

    let clone = Arc::clone(&app);
    evt_src.on_message(move |msg| {
        //let clone = Arc::clone(&app);
        //println!("new message {}", msg.data);
        let json: Vec<String> = serde_json::from_str(&msg.data).unwrap();
        clone.lock().unwrap().items = json;
    });

    let app2 = Arc::clone(&app);
    thread::spawn(|| {
        draw(app2);
    });

    loop {
        match stdin.next() {
            Some(Ok(b'q')) => {
                break;
            }
            Some(Ok(b'j')) => {
                let mut locked_app = app.lock().unwrap();
                if locked_app.selected < locked_app.items.len() - 1 {
                    locked_app.selected += 1;
                }
            }
            Some(Ok(b'k')) => {
                let mut locked_app = app.lock().unwrap();
                if locked_app.selected > 0 {
                    locked_app.selected -= 1;
                }
            }
            Some(Ok(b'r')) => {
                println!("refresh");
            }
            _ => {}
        }
    }

    Ok(())
}
