use std::error::Error;
use neo4rs::{query, Node};
use neo4rs::Graph;
use serde::Serialize;

pub struct GraphBackend {
    graph: Graph,
}

#[derive(Serialize)]
pub struct Message {
    id: String,
    text: String
}

impl GraphBackend {
    pub(crate) async fn new(uri: String, user: String, pass: String) -> Self {
        let graph = Graph::new(&uri, user, pass).unwrap();
        Self { graph }
    }

    pub(crate) async fn get_messages(&self) -> Result<Vec<Message>, Box<dyn Error>> {
        let mut result = self.graph.execute(
            query("MATCH (n:Message) RETURN n")
        ).await?;

        let mut messages: Vec<Message> = vec![];
        while let Some(row) = result.next().await? {
            let node: Node = row.get("n")?;
            let id: String = node.get("id")?;
            let text: String = node.get("text")?;
            messages.push( Message {
                id,
                text
            });
        }
        Ok(messages)
    }

    pub(crate) async fn update_text(&self, id: &str, text: &str) -> Result<Vec<Message>, Box<dyn Error>> {
        let mut result = self.graph.execute(
            query("MATCH (n:Message) WHERE n.id=$id SET n.text=$text RETURN n")
                .param("id", id)
                .param("text", text)
        ).await?;

        let mut messages: Vec<Message> = vec![];
        while let Some(row) = result.next().await? {
            let node: Node = row.get("n")?;
            let id: String = node.get("id")?;
            let text: String = node.get("text")?;
            messages.push( Message {
                id,
                text
            });
        }
        Ok(messages)
    }
}