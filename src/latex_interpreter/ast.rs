use std::sync::{Arc, Mutex};

type NodePtr = Arc<Mutex<Node>>;

pub enum NodeType {
    Document,
    Words,
    LineBreak,

    InlineMath,
    Displaymath,

    Envr,  // environment
    
    Command,
    Comment,
}

pub struct Node {
    lexeme: String,
    args: Vec<NodePtr>,
    children: Vec<NodePtr>,
}
