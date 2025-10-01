use std::convert;
use std::fmt;
use std::sync::{Arc, Mutex};

pub type NodePtr = Arc<Mutex<Node>>;

#[derive(Debug)]
pub enum NodeType {
    Passage,   // A passage consisists of many paragraphs
    Paragraph, // A paragraph consists of many Words, operations, etc
    Word,
    Operation,       // parsing a^b a_c
    Ampersand,       // & are used for alignment in Latex
    DoubleBackSlash, //  \\

    Command,
    CurlyBracketArg, // {para}
    SquareBracketArg,

    InlineMath,
    DisplayMath,

    Envr, // environment

    Comment,
}

#[derive(Debug)]
pub struct Node {
    pub lexeme: String,
    pub node_type: NodeType,
    pub children: Vec<NodePtr>,
}

impl Node {
    pub fn new(lexeme: &str, node_type: NodeType) -> Self {
        let lexeme = lexeme.to_string();
        Node {
            lexeme,
            node_type,
            children: vec![],
        }
    }

    pub fn attach(&mut self, ptr: NodePtr) {
        self.children.push(ptr);
    }

    pub fn empty_passage_ptr() -> NodePtr {
        Arc::new(Mutex::new(Node {
            lexeme: String::new(),
            node_type: NodeType::Passage,
            children: vec![],
        }))
    }
    pub fn empty_paragraph_ptr() -> NodePtr {
        Arc::new(Mutex::new(Node {
            lexeme: String::new(),
            node_type: NodeType::Paragraph,
            children: vec![],
        }))
    }

    pub fn dummy_ptr() -> NodePtr {
        Arc::new(Mutex::new(Node {
            lexeme: String::new(),
            node_type: NodeType::Paragraph,
            children: vec![],
        }))
    }

    pub fn lexeme_from_nodeptr(node: NodePtr) -> String {
        let node = node.lock().unwrap();
        node.lexeme.to_string()
    }

    pub fn get_string_content_recur(&self) -> String {
        let mut ret: String = String::new();
        ret.push_str(&self.lexeme);

        for i in self.children.iter() {
            let tmp = i.lock().unwrap();
            ret.push_str(&tmp.get_string_content_recur());
        }

        ret
    }

    pub fn get_string_content_recur_nodeptr(node: NodePtr) -> String {
        let node = node.lock().unwrap();
        let mut ret: String = String::new();
        ret.push_str(&node.lexeme);

        for i in node.children.iter() {
            let tmp = i.lock().unwrap();
            ret.push_str(&tmp.get_string_content_recur());
        }

        ret
    }

    pub fn dummy() -> Node {
        Node {
            lexeme: String::new(),
            node_type: NodeType::Paragraph,
            children: vec![],
        }
    }
}

/// Expected to display ast node with tree format (like the output of bash tree)
/// like these
/// Paragraph()
/// ├── Paragraph()
/// │   └── Paragraph()
/// │       └── Paragraph()
/// └── Paragraph()
///     ├── Paragraph()
///     │   ├── Paragraph()
///     │   └── Paragraph()
///     ├── Paragraph()
///     │   ├── Paragraph()
///     │   └── Paragraph()
///     └── Paragraph()
///         ├── Paragraph()
///         └── Paragraph()
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn aux(node: &Node) -> Vec<String> {
            let mut ret: Vec<String> = vec![];
            ret.push(format!("{:?}({})", node.node_type, node.lexeme));

            let children = &node.children;
            for i in 0..children.len() {
                let child = children[i].lock().unwrap();
                let child_display = aux(&*child);
                if i != children.len() - 1 {
                    for j in 0..child_display.len() {
                        if j == 0 {
                            ret.push(format!("├── {}", child_display[j]));
                        } else {
                            ret.push(format!("│   {}", child_display[j]));
                        }
                    }
                } else {
                    for j in 0..child_display.len() {
                        if j == 0 {
                            ret.push(format!("└── {}", child_display[j]));
                        } else {
                            ret.push(format!("    {}", child_display[j]));
                        }
                    }
                }
            }

            ret
        }

        let mut dis = String::new();
        let vec_str = aux(self);
        for i in 0..vec_str.len() {
            dis.push_str(&vec_str[i]);
            if i != vec_str.len() {
                dis.push_str("\n");
            }
        }
        write!(f, "{}", dis)
    }
}

impl convert::Into<NodePtr> for Node {
    fn into(self) -> NodePtr {
        Arc::new(Mutex::new(self))
    }
}
