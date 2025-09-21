use super::ast::{Node, NodePtr, NodeType};
use super::scanner::{scan, Token, TokenType};
use std::convert;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

/// The parser parse vec of token types in AST(abstract syntax tree)
/// The parser will delete and arrange tokens, but will not modify the lexeme of any token
///
/// This is the parse grammar
/// Note * denotes 0 or more, + denotes 1 or more
///
/// Passage -> Passage Paragraph \n\n+ (two or more consective line breaks)
/// Paragraph -> Paragraph E
/// E -> Word
/// E -> Space+  // multiple spaces are treated as one
/// E -> Operator
/// Space -> \n  // a single line break is a space
/// Space -> ' ' | '\t'+  // one or more consecutive space (or tabs) is considered as a single space
/// BraceArg -> {Paragraph}
/// BracketArg -> [Paragraph]

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

    fn unexpected_eof() -> Self {
        ParseError {
            details: format!("Unexpected End of Line!"),
            error_type: ErrorType::UnexpectedEOF,
        }
    }

    fn unexpected_eof_internal(info: &str) -> Self {
        ParseError {
            details: format!(
                "Unexpected End of Line! Likely internal bug. Info: {}",
                info
            ),
            error_type: ErrorType::UnexpectedEOF,
        }
    }

    fn miss_match_type_interanl(expected: TokenType, found: TokenType, info: &str) -> Self {
        ParseError {
            details: format!(
                "Expected {:?}, found {:?}. Likely Internal Bug. Info: {}",
                expected, found, info
            ),
            error_type: ErrorType::UnexpectedToken,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {} ({:?})", self.details, self.error_type)
    }
}

// Use the default implementations
impl Error for ParseError {}

/// This is the main function of this file
pub fn parse(input: &[Token]) -> Result<NodePtr, Box<dyn Error>> {
    let mut pos: usize = 0;
    let root_ptr = Node::empty_passage_ptr();

    let mut root = root_ptr.lock().unwrap();
    while pos < input.len() {
        root.attach(parse_paragraph(input, &mut pos)?);

        if pos < input.len() && poke2(input, pos, TokenType::Newline, TokenType::Newline)? {
            parse_newline(input, &mut pos)?;
        }
    }

    Ok(root_ptr.clone())
}

pub fn eat(
    input: &mut [Token],
    pos: &mut usize,
    t_type: TokenType,
) -> Result<Token, Box<dyn Error>> {
    if input.len() <= *pos {
        return Err(
            ParseError::unexpected_eof_internal("Error generated from function eat").into(),
        );
    }
    if input[*pos].token_type != t_type {
        return Err(ParseError::miss_match_type_interanl(
            t_type,
            input[*pos].token_type.clone(),
            "Error generated from function eat.",
        )
        .into());
    }
    *pos += 1;
    Ok(input[*pos].clone())
}

/// Check if input[pos] == token_type_1, return Ok(true) if it is, Ok(false) if it is not
pub fn poke(input: &[Token], pos: usize, token_type_1: TokenType) -> Result<bool, Box<dyn Error>> {
    if input.len() <= pos {
        return Err(
            ParseError::unexpected_eof_internal("Error generated from function poke").into(),
        );
    }
    if input[pos].token_type == token_type_1 {
        return Ok(true);
    }

    Ok(false)
}

/// Check if input[pos] == token_type_1 and input[pos + 1] == token_type_2
/// return Ok(true) if both types match, Ok(false) if one of them does not
pub fn poke2(
    input: &[Token],
    pos: usize,
    token_type_1: TokenType,
    token_type_2: TokenType,
) -> Result<bool, Box<dyn Error>> {
    if input.len() <= pos + 1 {
        return Err(
            ParseError::unexpected_eof_internal("Error generated from function poke2").into(),
        );
    }
    if input[pos].token_type == token_type_1 && input[pos + 1].token_type == token_type_2 {
        return Ok(true);
    }

    Ok(false)
}

fn parse_newline(input: &[Token], pos: &mut usize) -> Result<(), Box<dyn Error>> {
    if *pos >= input.len() {
        return Err(ParseError::unexpected_eof_internal(
            "Error generated from function parse_newline",
        )
        .into());
    }
    if input[*pos].token_type != TokenType::Newline {
        return Err(ParseError::miss_match_type_interanl(
            TokenType::Space,
            input[*pos].token_type.clone(),
            "Error generated from function parse_newline",
        )
        .into());
    }
    while input[*pos].token_type == TokenType::Newline {
        *pos += 1;
    }

    Ok(())
}
fn parse_space(input: &[Token], pos: &mut usize) -> Result<(), Box<dyn Error>> {
    if *pos >= input.len() {
        return Err(ParseError::unexpected_eof_internal(
            "Error genrated from function parse_space",
        )
        .into());
    }
    if input[*pos].token_type != TokenType::Space && input[*pos].token_type != TokenType::Newline {
        panic!(
            "Expected: Space or NewLine token, Found: {:?}",
            input[*pos].token_type
        );
    }

    while input[*pos].token_type == TokenType::Space || input[*pos].token_type == TokenType::Newline
    {
        // In the case of two consecutive newlines, just return
        if input[*pos].token_type == TokenType::Newline {
            if *pos + 1 < input.len() && input[*pos + 1].token_type == TokenType::Newline {
                return Ok(());
            }
        }
        *pos += 1;
    }

    Ok(())
}

// This is the main parse logic, as the whole latex file is a paragraph
// We are implementing a simple LL(1) recursive parser
fn parse_paragraph(input: &[Token], pos: &mut usize) -> Result<NodePtr, Box<dyn Error>> {
    let ret: Arc<Mutex<Node>> = Node::empty_paragraph_ptr();
    let mut paragraph = ret.lock().unwrap();

    while *pos < input.len() {
        let cur_token = &input[*pos];
        match cur_token.token_type {
            TokenType::Word => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Word).into());
                *pos += 1;
            }
            TokenType::Comment => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Comment).into());
                *pos += 1;
            }
            TokenType::Space => {
                parse_space(input, pos)?;
                // between words token, we assume there is a space
                // The space token is for manually created space like `\ `
                // NOTE:: maybe we can simply ignore space token type in the
                // scanner
            }
            TokenType::Newline => {
                // In case of two consective newline, return ret
                // and let the parse function to handle
                if poke2(input, *pos, TokenType::Newline, TokenType::Newline)? {
                    return Ok(ret.clone());
                }
                parse_space(input, pos)?;
            }
            TokenType::Ampersand => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Ampersand).into());
                *pos += 1;
            }

            // TODO:
            _ => {}
        }
    }

    Ok(ret.clone())
}

#[cfg(test)]
mod test {
    #[test]
    fn parser_try() {
        use crate::latex_interpreter::*;
        let input = r##"Hello!     Junlu&! 

New paragraph!"##;
        let input = scanner::scan(input);
        let ast = parser::parse(&input).unwrap();

        println!("{}", ast.lock().unwrap());
    }
}
