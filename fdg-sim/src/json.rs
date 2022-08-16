//!
//! Generate graphs from json formatted in the [json graph specification](https://github.com/jsongraph/json-graph-specification).
//! Hyperedges aren't implemented, but basic graphs like should work:
//! ```json
//! {
//!     "graph": {
//!         "nodes": {
//!             "A": {
//!                 "label": "label for A is moved to data section in node"
//!             },
//!             "B": {
//!                 "metadata": {
//!                     "something": "here"
//!                 }
//!             },
//!             "C": {}
//!         },
//!         "edges": [
//!             {
//!                 "source": "A",
//!                 "target": "B",
//!                 "metadata": 123451.45
//!             },
//!             {
//!                 "source": "B",
//!                 "target": "C",
//!                 "metadata": "just a string here!"
//!             },
//!             {
//!                 "source": "C",
//!                 "target": "A",
//!                 "metadata": { "key": "value" }
//!             }
//!         ]
//!     }
//! }
//! ```

use std::{collections::HashMap, error::Error, fmt};

use petgraph::{graph::NodeIndex, visit::{IntoEdgeReferences, EdgeRef}};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

use crate::{ForceGraph, ForceGraphHelper};

/// Possible errors returned by the functions in the module.
#[derive(Debug)]
pub enum JsonError {
    BadFormatting(serde_json::Error),
    HyperEdges,
    NodeNotFound(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadFormatting(err) => write!(f, "Input not JSON: {err}"),
            Self::HyperEdges => write!(f, "Graphs with hyperedges are not supported"),
            Self::NodeNotFound(n) => write!(f, "Node {n} not defined in graph"),
        }
    }
}

impl Error for JsonError {}

#[derive(Serialize, Deserialize, Debug)]
struct JsonGraph {
    pub graph: InnerJsonGraph,
}

#[derive(Serialize, Deserialize, Debug)]
struct InnerJsonGraph {
    pub nodes: HashMap<String, JsonNode>,
    pub edges: Option<Vec<JsonEdge>>,
    // will return an error if you use hyperedges
    #[serde(rename = "hyperedges")]
    pub _hyperedges: Option<Value>,
    #[serde(rename = "id")]
    pub _id: Option<Value>,
    #[serde(rename = "type")]
    pub _type: Option<Value>,
    #[serde(rename = "label")]
    pub _label: Option<Value>,
    #[serde(rename = "directed")]
    pub _directed: Option<Value>,
    #[serde(rename = "metadata")]
    pub _metadata: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonNode {
    pub label: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonEdge {
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub metadata: Value,
}

/// Create a json value from a [`ForceGraph`].
pub fn graph_to_json<N: Serialize, E: Serialize>(graph: &ForceGraph<N, E>) -> Result<Value, serde_json::Error> {
    let mut nodes: HashMap<String, Value> = HashMap::new();
    let mut edges: Vec<JsonEdge> = Vec::new();

    for node in graph.node_weights() {
        let metadata: Value = serde_json::to_value(&node.data)?;

        nodes.insert(node.name.to_owned(), metadata);
    }

    for edge in graph.edge_references() {
        let edge = JsonEdge {
            source: (&graph[edge.source()]).name.to_owned(),
            target: (&graph[edge.target()]).name.to_owned(),
            metadata: serde_json::to_value(edge.weight())?,
        };

        edges.push(edge);
    }

    let mut outer_graph: Map<String, Value> = Map::new();

    let mut inner_graph: Map<String, Value> = Map::new();

    inner_graph.insert("nodes".to_string(), serde_json::to_value(&nodes)?);
    inner_graph.insert("edges".to_string(), serde_json::to_value(&edges)?);

    outer_graph.insert("graph".to_string(), serde_json::to_value(&inner_graph)?);

    Ok(Value::Object(outer_graph))
}

/// Get a [`ForceGraph`] from a json string.
pub fn graph_from_json(json: impl AsRef<str>) -> Result<ForceGraph<Value, Value>, JsonError> {
    let mut graph: ForceGraph<Value, Value> = ForceGraph::default();

    let json: JsonGraph = match serde_json::from_str(json.as_ref()) {
        Ok(json) => json,
        Err(err) => return Err(JsonError::BadFormatting(err)),
    };

    if json.graph._hyperedges.is_some() {
        return Err(JsonError::HyperEdges);
    }

    for (name, data) in json.graph.nodes {
        let data = match serde_json::to_value(&data) {
            Ok(data) => data,
            Err(err) => return Err(JsonError::BadFormatting(err)),
        };

        graph.add_force_node(name, data.into());
    }

    if let Some(edges) = json.graph.edges {
        for edge in edges {
            let source = match index_from_name(&edge.source, &graph) {
                Some(source) => source,
                None => return Err(JsonError::NodeNotFound(edge.source)),
            };

            let target = match index_from_name(&edge.target, &graph) {
                Some(source) => source,
                None => return Err(JsonError::NodeNotFound(edge.target)),
            };

            graph.add_edge(source, target, edge.metadata);
        }
    }

    Ok(graph)
}

fn index_from_name(name: impl AsRef<str>, graph: &ForceGraph<Value, Value>) -> Option<NodeIndex> {
    let name = name.as_ref().to_string();

    for idx in graph.node_indices() {
        if name == graph[idx].name {
            return Some(idx);
        }
    }

    None
}