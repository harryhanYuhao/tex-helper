use super::ast::{Node, NodePtr, NodeType}; 
use super::scanner::{TokenType, Token, scan};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::fmt;

/// The parse grammar
/// 
/// Note * denotes 0 or more, + denotes 1 or more
///
/// Paragraph -> Paragraph E
/// E -> Word 
/// E -> Space+  // multiple spaces are treated as one 
/// E -> LineBreak+  // multiple line breaks are treated as one 
/// E -> Operator

#[derive(Debug)]
enum ErrorType {
    UnexpectedToken,
    UnexpectedEOF,
    InvalidSyntax,
}


#[derive(Debug)]
struct ParseError {
    details: String,
    error_type: ErrorType,
}

impl ParseError {
    fn new(details: &str, error_type: ErrorType) -> Self {
        let details: String = details.to_string();
        ParseError {
            details,
            error_type,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {} ({:?})", self.details, self.error_type)
    }
}

// Use default implementations
impl Error for ParseError { }

pub fn parse(input: &mut [Token]) -> Result<NodePtr, Box<dyn Error>> {

    Ok(Node::dummy_ptr())
}

pub fn eat(input: &mut[Token], pos: &mut usize, t_type: TokenType) -> Result<Token, Box<dyn Error>> {
    if input.len() <= *pos {
        return Err(Box::new(ParseError::new("Trying to eat at end of input", ErrorType::UnexpectedEOF)));
    }
    if input[*pos].token_type != t_type {
        return Err(Box::new(ParseError::new(&format!("Expected {:?}, found {:?}", t_type, input[0].token_type), ErrorType::UnexpectedToken)));
    }
    *pos += 1;
    Ok(input[*pos].clone())
}

// This is the main parse logic, as the whole latex file is a paragraph
// We are implementing a simple LL(1) recursive parser
pub fn parse_paragraph(input: &[Token]) -> Result<NodePtr, Box<dyn Error>> {
    let ret: Arc<Mutex<Node>> = Node::empty_paragraph_ptr();
    let mut pos: usize = 0;

    while pos < input.len() {
        // TODO::
        break;
    }

    Ok(Node::dummy_ptr())
}
