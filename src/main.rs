#![allow(unused_imports)]
#![allow(dead_code)]
mod display;
mod util;

use crate::{
    display::{read_node_data, ui, App, Node, Nodes},
    util::event::{Event, Events},
};
use argh::FromArgs;
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

/// CLI
#[derive(Debug, FromArgs)]
struct Cli {
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();

    let events = Events::with_config();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // fetch node data
    let nodes: Nodes = read_node_data()?;

    /*
    let node_names = nodes
        .iter()
        .map(|node: &Node| node.name.as_str())
        .collect::<Vec<&str>>();
    */

    let node_names: Vec<&str> = Vec::new();

    let mut app = App::new(
        "nodewatch",
        cli.enhanced_graphics,
        nodes,
        node_names.to_vec(),
    );

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char(c) => {
                    app.on_key(c);
                }
                Key::Left => {
                    app.on_left();
                }
                Key::Right => {
                    app.on_right();
                }
                _ => {}
            },
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
