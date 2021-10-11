/// sandbox is a helper crate for combining json data but may ultimately 
/// replace main.rs as the CLI root

#![allow(dead_code)]
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{read_to_string, OpenOptions},
    io::Write,
    time::Instant,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawNodes {
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    aggregate_by_country: Vec<AggregateByCountry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AggregateByCountry {
    name: String,
    count: u64,
}

type Countries = Vec<Country>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Country {
    name: String,
    #[serde(rename = "country_code")]
    country_code: String,
    capital: Option<String>,
    latlng: Vec<f64>,
    timezones: Vec<String>,
}

#[derive(Debug)]
struct File {
    name: String,
}

type Nodes = Vec<Node>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub name: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    pub capital: String,
    pub count: u64,
    pub coordinates: Vec<f64>,
}

fn raw_nodes() -> Result<Vec<AggregateByCountry>, Box<dyn Error>> {
    let data = read_to_string("raw_nodes.json")?;
    let raw_nodes: RawNodes = serde_json::from_str(&data)?;
    Ok(raw_nodes.data.aggregate_by_country)
}

fn countries() -> Result<Countries, Box<dyn Error>> {
    let data = read_to_string("countries.json")?;
    let countries: Countries = serde_json::from_str(&data)?;
    Ok(countries)
}

fn print_countries() {
    let countries: Countries = countries().unwrap();
    for country in countries.iter() {
        {
            let lat: f64 = country.latlng[0];
            let long: f64 = country.latlng[1];

            let mut capital = String::new();

            match &country.capital {
                Some(value) => capital = value.to_owned(),
                None => (),
            };

            println!(
                "country: {}\ncountry_code: {}\ncapital: {}\nlat_long: ({},{})\n-----",
                country.name, country.country_code, capital, lat, long
            );
        };
    }
}

pub fn combine() -> Nodes {
    let raw_nodes: Vec<AggregateByCountry> = raw_nodes().unwrap();
    let countries: Countries = countries().unwrap();
    let mut nodes: Vec<Node> = Vec::new();

    for country in countries.iter() {
        for raw_node in raw_nodes.iter() {
            if country.name == raw_node.name {
                let lat: f64 = country.latlng[0];
                let long: f64 = country.latlng[1];

                let mut capital = String::new();

                match &country.capital {
                    Some(value) => capital = value.to_owned(),
                    None => (),
                };
                nodes.push(Node {
                    name: country.name.clone(),
                    country_code: country.country_code.clone(),
                    capital: capital,
                    count: raw_node.count,
                    coordinates: vec![lat, long],
                });
            }
        }
    }
    nodes
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("nodewatch")
        .version("0.1.0")
        .author("jon roethke <jon@chainsafe.io>")
        .about("nodewatch")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .help("prints prettified output of combining data")
                .requires("combine"),
        )
        .arg(
            Arg::with_name("render")
                .short("r")
                .long("render")
                .takes_value(false)
                .help("render the nodewatch terminal GUI"),
        )
        .arg(
            Arg::with_name("combine")
                .short("c")
                .long("combine")
                .takes_value(false)
                .help("combine raw_nodes data with static country coordinates"),
        )
        .get_matches();

    // flag checks
    let verbose_flag: bool = matches.is_present("verbose");
    let render_flag: bool = matches.is_present("render");
    let combine_flag: bool = matches.is_present("combine");

    // check if combine selected
    if combine_flag {
        // timer
        let now = Instant::now();

        let nodes: Nodes = combine();
        {
            let mut file = OpenOptions::new()
                .read(false)
                .write(true)
                .append(false)
                .create(true)
                .open("nodes.json")?;

            let nodes_str: String = serde_json::to_string(&nodes)?;

            file.write_all(nodes_str.as_bytes())?;

            println!("successfully written to file: nodes.json")
        }

        // if verbose selected with combine
        if verbose_flag {
            println!("nodes: {}\n\n:::::::::::\n", nodes.len());
            for node in nodes {
                println!(
                "country: {}\ncountry_code: {}\ncapital: {}\ncount: {}\nlat_long: ({},{})\n-----",
                node.name,
                node.country_code,
                node.capital,
                node.count,
                node.coordinates[0],
                node.coordinates[1]
            );
            }
        }
        println!("done: {}ms", now.elapsed().as_millis());
    } else if render_flag {
        todo!();
    } else {
        println!("{}", matches.usage());
    }

    Ok(())
}
