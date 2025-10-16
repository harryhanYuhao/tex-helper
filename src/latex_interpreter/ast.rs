//! This file defines the node which constitutes the
//! abstract Syntax Tree for Latex
//! To comply with rust's ownership system, everything is wrapped in Arc<Mutex<>>
//!
//! Passage and paragraph are solely for structuring the AST, these two are called "container
//! nodes"
//! the rests are "content nodes"
use std::convert;
use std::fmt;
use std::sync::{Arc, Mutex};

pub type NodePtr = Arc<Mutex<Node>>;

#[derive(Debug, Clone, PartialEq, Eq)]
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

/// A Node in AST
/// Node itself does not keep track of its parent, so by itself it is not enougth to traverse the
/// tree with backtracking.
/// For traversing the tree, use the WALKER struct
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

    pub fn get_children_string_content_recur(&self) -> String {
        let mut ret: String = String::new();
        for i in self.children.iter() {
            let tmp = i.lock().unwrap();
            ret.push_str(&tmp.get_string_content_recur());
        }
        ret
    }


    /// Recursively append the lexemes of self and all its children
    /// depth first
    /// EG  for tree 
    /// A("A")
    /// ├── B("B")
    /// │   ├── C("C")
    /// │   └── D("D")
    /// └── E("E")
    /// the output is "ABCDE"
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
        node.get_string_content_recur()
    }

    pub fn dummy() -> Node {
        Node {
            lexeme: String::new(),
            node_type: NodeType::Paragraph,
            children: vec![],
        }
    }

    pub fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }
    
    pub fn get_node_type_nodeptr(node: NodePtr) -> NodeType {
        let node = node.lock().unwrap();
        node.get_node_type().clone()
    }

    pub fn get_children(&self) -> &[NodePtr] {
        &self.children
    }

    pub fn get_children_nodeptr(node: NodePtr) -> Vec<NodePtr> {
        let node = node.lock().unwrap();
        node.get_children().to_vec()
    }

    pub fn get_nth_child(&self, id: usize) -> Option<NodePtr> {
        if self.children.len() <= id {
            return None;
        }
        Some(self.children[id].clone())
    }

    pub fn get_nth_child_nodeptr(node: NodePtr, id: usize) -> Option<NodePtr> {
        let node = node.lock().unwrap();
        node.get_nth_child(id)
    }

    pub fn is_container(&self) -> bool {
        match self.node_type {
            NodeType::Passage | NodeType::Paragraph => true,
            _ => false,
        }
    }

    pub fn is_container_nodeptr(node: NodePtr) -> bool {
        let node = node.lock().unwrap();
        node.is_container()
    }

    pub fn is_content(&self) -> bool {
        !self.is_container()
    } 

    pub fn is_content_nodeptr(node: NodePtr) -> bool {
        let node = node.lock().unwrap();
        node.is_content()
    }

}

#[cfg(test)]
mod test_node {
    #[test]
    /// We 
    fn test_get_string_content_recur() {
        use super::*;
        let mut a = Node::new("A", NodeType::Paragraph);
        let mut b = Node::new("B", NodeType::Paragraph);
        let c = Node::new("C", NodeType::Paragraph);
        let d = Node::new("D", NodeType::Paragraph);
        let e = Node::new("E", NodeType::Paragraph);

        b.attach(c.into());
        b.attach(d.into());
        a.attach(b.into());
        a.attach(e.into());

        assert_eq!(a.get_string_content_recur(), "ABCDE");
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

/// Walker Struct for traversing the AST
/// usize keeps the index of the child
/// eg, for tree
/// Root
/// ├── A
/// │   ├── A1
/// │   ├── A2
/// │   └── A3
/// └── B
///     ├── B1
///     ├── B2
///     └── B3
///
///
/// at A1, the stack is [(Root, 0), (A, 0)]
/// at B3, the stack is [(Root, 1), (B, 2)] (we are at the second child of B)
///
/// In general, if we have [(a1, b1), (a2, b2) ...  (an, bn)], then b1-th child of a1 is a2,
/// b2-th child of a2 is a3  ....
/// and the current location of the walker is the bn-th child of an.
pub struct Walker {
    root: NodePtr,
    stack: Vec<(NodePtr, usize)>,
}

impl Walker {
    pub fn from_root(root: NodePtr) -> Self {
        Walker {
            root,
            stack: vec![],
        }
    }

    /// returns the current location of the Walker
    /// If stack is empty, return root
    /// otherwise, return the node which the last entry of the stacks points to.
    ///
    /// MAY PANIC!!
    pub fn cur_loc(&self) -> NodePtr {
        if self.stack.is_empty() {
            return self.root.clone();
        }
        let (last_par, id) = self.stack[self.stack.len() - 1].clone();
        match Node::get_nth_child_nodeptr(last_par, id) {
            Some(s) => return s,
            None => panic!("Internal Error!"),
        }
    }

    /// Return the Some(node), where node is the next node at the same level
    /// and sharing the same parent node with self
    ///
    /// Return none if there parent does not contain any more node
    ///
    /// eg
    ///
    /// Root
    /// ├── A
    /// │   ├── A1
    /// │   ├── A2
    /// │   └── A3
    /// └── B
    ///     ├── B1
    ///     ├── B2
    ///     └── B3
    ///
    /// A.next_sibling() == Some(B)
    /// A1.next_sibling() == Some(A2)
    /// root.next_sibling == None
    /// B.next_sibling == None
    pub fn next_sibling(&self) -> Option<NodePtr> {
        if self.stack.is_empty() {
            // in this case we check if root has child
            return None;
        }
        let (parent, index) = self.stack[self.stack.len() - 1].clone();
        Node::get_nth_child_nodeptr(parent, index + 1)
    }

    pub fn first_child(&self) -> Option<NodePtr> {
        let cur = self.cur_loc();
        Node::get_nth_child_nodeptr(cur, 0)
    }

    pub fn next_content_node(&mut self) -> Option<NodePtr> {
        let mut is_root: bool = false;
        let node = if self.stack.is_empty() {
            is_root = true;
            self.root.clone()
        } else {
            let (parent, index) = self.stack[self.stack.len() - 1].clone();
            Node::get_nth_child_nodeptr(parent, index).unwrap()
        };
        None
    }
}
