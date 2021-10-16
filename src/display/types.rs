use serde::{Deserialize, Serialize};

/// Nodes

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawNodes {
    pub data: NodesData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodesData {
    #[serde(rename = "aggregate_by_country")]
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

/// Clients
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawClients {
    pub data: ClientData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientData {
    #[serde(rename = "aggregate_by_agent_name")]
    pub aggregate_by_agent_name: Vec<AggregateByAgentName>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateByAgentName {
    pub name: String,
    pub count: u32,
}

pub type Clients = Vec<AggregateByAgentName>;
