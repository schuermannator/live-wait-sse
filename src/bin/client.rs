use sse_client::EventSource;
use std::io::{Read, stdin};
use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Sender, Receiver};
use std::{error::Error, io, thread};
use termion::{event::Key, input::TermRead, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
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
            items: vec![],
            selected: 0,
        }
    }
}

fn draw(app: Arc<Mutex<App>>, chan: Receiver<bool>) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    loop {
        chan.recv().unwrap();
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
}

fn main() -> Result<(), Box<dyn Error>> {
    //let mut stdin = async_stdin().bytes();
    let app = Arc::new(Mutex::new(App::new()));
    let evt_src = EventSource::new("https://oh.zvs.io/sse").unwrap();
    let stdin = stdin();

    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let thread_tx = tx.clone();
    let clone = Arc::clone(&app);
    evt_src.on_message(move |msg| {
        //let clone = Arc::clone(&app);
        println!("new message {}", msg.data);
        let json: Vec<String> = serde_json::from_str(&msg.data).unwrap();
        clone.lock().unwrap().items = json;
        thread_tx.send(true).unwrap();
    });

    let app2 = Arc::clone(&app);
    thread::spawn(|| {
        draw(app2, rx).unwrap();
    });

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break;
            }
            Key::Char('j') => {
                let mut locked_app = app.lock().unwrap();
                if locked_app.selected < locked_app.items.len() - 1 {
                    locked_app.selected += 1;
                }
                tx.send(true).unwrap();
            }
            Key::Char('k') => {
                let mut locked_app = app.lock().unwrap();
                if locked_app.selected > 0 {
                    locked_app.selected -= 1;
                }
                tx.send(true).unwrap();
            }
            Key::Char('r') => {
                println!("refresh");
                tx.send(true).unwrap();
            }
            _ => {}
        }
    }

    Ok(())
}
