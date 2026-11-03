use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MeshMessage {
    pub from: String,
    pub to: String,
    pub body: Vec<u8>,
}

pub trait MeshTransport {
    fn send(&mut self, msg: MeshMessage);
    fn recv(&mut self, node: &str) -> Vec<MeshMessage>;
}

pub struct InMemoryMesh {
    inbox: HashMap<String, Vec<MeshMessage>>,
}

impl InMemoryMesh {
    pub fn new() -> Self {
        Self { inbox: HashMap::new() }
    }
}

impl MeshTransport for InMemoryMesh {
    fn send(&mut self, msg: MeshMessage) {
        self.inbox.entry(msg.to.clone()).or_default().push(msg);
    }

    fn recv(&mut self, node: &str) -> Vec<MeshMessage> {
        self.inbox.remove(node).unwrap_or_default()
    }
  }
