use crate::display::types::{Clients, Node, Nodes};
use crate::display::util::{StatefulList, TabsState};
use std::{error::Error, fs};

pub fn read_client_data() -> Result<Clients, Box<dyn Error>> {
    let data = fs::read_to_string("clients.json")?;
    let clients: Clients = serde_json::from_str(&data)?;

    Ok(clients)
}

pub fn read_node_data() -> Result<Nodes, Box<dyn Error>> {
    let data = fs::read_to_string("nodes.json")?;
    let nodes: Nodes = serde_json::from_str(&data)?;

    Ok(nodes)
}

pub fn read_node_countries() -> Result<Vec<String>, Box<dyn Error>> {
    let data = fs::read_to_string("node_countries.json")?;
    let node_countries: Vec<String> = serde_json::from_str(&data)?;

    Ok(node_countries)
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub raw_nodes: StatefulList<&'a str>,
    pub nodes: Vec<Node>,
    pub clients: Clients,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        nodes: Vec<Node>,
        node_names: Vec<&'a str>,
        clients: Clients,
    ) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["map", "list", "clients"]),
            show_chart: true,
            raw_nodes: StatefulList::with_items(node_names),
            nodes: nodes,
            clients: clients,
            enhanced_graphics,
        }
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'h' => {
                self.on_left();
            }
            'l' => {
                self.on_right();
            }
            _ => {}
        }
    }
}
