//File for json parsing and translating. 
// Resources:
    // https://users.rust-lang.org/t/getting-started-with-rust-and-json/128674
    // https://thelinuxcode.com/rust-json-example/ 
    // https://blog.logrocket.com/json-and-rust-why-serde_json-is-the-top-choice/
    // ChatGPT --> dubugging

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_json;
use crate::graph::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};

// structure to hold the JSON data
#[derive(Debug, Deserialize)]
pub struct JsonData {
    pub username: String,
    pub mutuals: i32,
}

#[derive(Serialize, Deserialize)]
struct JsonNode<N> {
    id: usize,
    data: N,
}

#[derive(Serialize, Deserialize)]
struct JsonEdge<W> {
    src: usize,
    dst: usize,
    weight: W,
}

#[derive(Serialize, Deserialize)]
pub struct JsonGraph<N, W> {
    directed: bool,
    nodes: Vec<JsonNode<N>>,
    edges: Vec<JsonEdge<W>>,
}

/* 
// function to read JSON data from a file
pub fn read_json(file: &str) -> Result<JsonData, Box<dyn Error>> {
    let data = fs::read_to_string(file)?;
    let file_data: JsonData = serde_json::from_str(&data)?;
    Ok(file_data)
}
*/
//translating the graph to json
impl<N, W> Graph<N, W>
where
    N: Clone + Serialize,
    W: Clone + Serialize,
{
    pub fn to_json_model(&self, directed: bool) -> JsonGraph<N, W> {
        let mut nodes = Vec::with_capacity(self.len());
        for u in 0..self.len() {
            nodes.push(JsonNode { id: u, data: self.node(u).clone() });
        }

        let mut edges = Vec::new();
        for u in 0..self.len() {
            for &(v, ref w) in self.neighbors(u) {
                // if treating graph as undirected, emit each unordered pair once
                if !directed && v < u {
                    continue;
                }
                edges.push(JsonEdge { src: u, dst: v, weight: w.clone() });
            }
        }

        JsonGraph { directed, nodes, edges }
    }
}

impl<N, W> Graph<N, W>
where
    N: Clone + DeserializeOwned,
    W: Clone + DeserializeOwned,
{
    pub fn from_json_model(jg: JsonGraph<N, W>) -> Self {
        let mut g = Graph::new();

        // Recreate nodes in the same order so ids match JSON ids.
        for n in &jg.nodes {
            let id = g.add_node(n.data.clone());
            debug_assert_eq!(id, n.id, "node ids not contiguous/order-preserving");
        }

        // Add edges. If the JSON described an undirected graph,
        // mirror edges here so your in-memory graph has both directions.
        for e in &jg.edges {
            g.add_edge(e.src, e.dst, e.weight.clone());
            if !jg.directed {
                g.add_edge(e.dst, e.src, e.weight.clone());
            }
        }

        g
    }
}

impl<N, W> Graph<N, W>
where
    N: Clone + Serialize + DeserializeOwned,
    W: Clone + Serialize + DeserializeOwned,
{
    pub fn write_json_pretty(&self, path: &str, directed: bool) -> Result<()> {
        let model = self.to_json_model(directed);
        let writer = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(writer, &model)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn read_json(path: &str) -> Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        let model: JsonGraph<N, W> = serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(Self::from_json_model(model))
    }
}
