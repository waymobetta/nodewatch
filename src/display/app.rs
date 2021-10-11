use crate::util::{StatefulList, TabsState};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawNodes {
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

pub type Nodes = Vec<Node>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub name: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub capital: String,
    pub count: u64,
    pub coordinates: Vec<f64>,
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
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        nodes: Vec<Node>,
        node_names: Vec<&'a str>,
    ) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["map", "list"]),
            show_chart: true,
            raw_nodes: StatefulList::with_items(node_names),
            nodes: nodes,
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
            _ => {}
        }
    }

    pub fn print_nodes(&mut self) {
        let nodes = read_node_data();
        println!("{:?}", nodes);
    }
}
