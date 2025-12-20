use std::error::Error;
use std::io;
use neo4rs::{query, Node, Txn};
use neo4rs::Graph;

pub struct GraphBackend {
    graph: Graph,
}

impl GraphBackend {
    pub(crate) async fn new(uri: String, user: String, pass: String) -> Self {
        let graph = Graph::new(&uri, user, pass).unwrap();
        Self { graph }
    }

    pub(crate) async fn test(self) -> Result<String, Box<dyn Error>> {
        let mut result = self.graph.execute(
            query("MATCH (n:Concept) RETURN n")
        ).await.unwrap();

        let mut output: String = "".to_string();
        while let Some(row) = result.next().await.unwrap() {
            let node: Node = row.get("n")?;
            let text: String = node.get("name")?;
            output  = output + "\n" + &text;
        }
        return Ok(output);
    }
}