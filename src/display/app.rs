use crate::util::{StatefulList, TabsState};
use serde::Deserialize;
use std::{error::Error, fs};

const RAW_NODES: [&str; 2] = ["United States", "Norway"];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub aggregate_by_country: Vec<AggregateByCountry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateByCountry {
    pub name: String,
    pub count: i64,
}

fn read_node_data() -> Result<Nodes, Box<dyn Error>> {
    let data = fs::read_to_string("nodes.json")?;
    let nodes: Nodes = serde_json::from_str(&data)?;

    Ok(nodes)
}

pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub coords: (f64, f64),
    pub status: &'a str,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub raw_nodes: StatefulList<&'a str>,
    pub servers: Vec<Server<'a>>,
    pub nodes: Vec<AggregateByCountry>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["map", "list"]),
            show_chart: true,
            raw_nodes: StatefulList::with_items(RAW_NODES.to_vec()),
            nodes: vec![],
            servers: vec![
                Server {
                    name: "NorthAmerica-1",
                    location: "New York City",
                    coords: (40.71, -74.00),
                    status: "Up",
                },
                Server {
                    name: "Europe-1",
                    location: "Paris",
                    coords: (48.85, 2.35),
                    status: "Failure",
                },
                Server {
                    name: "SouthAmerica-1",
                    location: "São Paulo",
                    coords: (-23.54, -46.62),
                    status: "Up",
                },
                Server {
                    name: "Asia-1",
                    location: "Singapore",
                    coords: (1.35, 103.86),
                    status: "Up",
                },
            ],
            enhanced_graphics,
        }
    }

    pub fn new_nodes(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["map", "list"]),
            show_chart: true,
            raw_nodes: StatefulList::with_items(RAW_NODES.to_vec()),
            servers: vec![],
            nodes: vec![
                AggregateByCountry {
                    name: String::from("United States"),
                    count: 950,
                },
                AggregateByCountry {
                    name: String::from("Norway"),
                    count: 8,
                },
            ],
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.raw_nodes.previous();
    }

    pub fn on_down(&mut self) {
        self.raw_nodes.next();
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
            _ => {}
        }
    }

    pub fn print_nodes(&mut self) {
        let nodes = read_node_data();
        println!("{:?}", nodes);
    }
}
