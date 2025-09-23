use super::ast::{Node, NodePtr, NodeType};
use super::scanner::{scan, Token, TokenType};
use std::convert;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

/// This is a naive LL(2) recursive parser for Latex
/// The parser parse vec of token types in AST(abstract syntax tree)
/// The parser will delete and arrange tokens, but will not modify the lexeme of any token
///
/// This is the parse grammar
/// Note * denotes 0 or more, + denotes 1 or more
///
/// Passage -> Passage Paragraph \n\n+ (two or more consective line breaks)
///
/// Paragraph -> Paragraph E
///
/// E -> Word
/// E -> Commant
/// E -> LoneCommand   // Commands without args
/// E -> SPACE* ( space are simple ignored)
///
/// Space -> \n  // a single line break is a space
/// Space -> ' ' | '\t'+  // one or more consecutive space (or tabs) is considered as a single space
///
///
/// E -> Operation
/// Operation -> Word Operator Word
/// Operation -> Word Operator BraceArg
///
/// E -> CommandWithArg
/// CommandWithArg -> LoneCommand (BraceArg | BracketArg)+
/// BraceArg -> {Paragraph}
/// BracketArg -> [Paragraph]

#[derive(Debug)]
enum ErrorType {
    UnexpectedToken,
    UnexpectedEOF,
    InvalidSyntax,
    UnexpectedRightBrace,
    UnexpectedRightBracket,
    UnexpectedSlashCloseBracket, // \] for ending math mode
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
    let mut prev_pos = pos; // For debug purpose

    while pos < input.len() {
        let paragraph = parse_paragraph(input, &mut pos)?;

        if poke(input, pos, TokenType::Newline) {
            root.attach(paragraph);
            pos += 1;
        } else if pos == input.len() {
            root.attach(paragraph);
        }

        // For debug purpose
        if prev_pos == pos {
            panic!("parse in infinite loop!")
        }
        prev_pos = pos;
    }

    Ok(root_ptr.clone())
}

/// Check if input\[pos\] == token_type_1, return Ok(true) if it is, Ok(false) if it is not
pub fn poke(input: &[Token], pos: usize, token_type_1: TokenType) -> bool {
    if input.len() <= pos {
        return false;
    }
    if input[pos].token_type == token_type_1 {
        return true;
    }

    false
}

/// Check if input[pos] == token_type_1 and input[pos + 1] == token_type_2
/// return Ok(true) if both types match, Ok(false) if one of them does not
pub fn poke2(
    input: &[Token],
    pos: usize,
    token_type_1: TokenType,
    token_type_2: TokenType,
) -> bool {
    if input.len() <= pos + 1 {
        return false;
    }
    if input[pos].token_type == token_type_1 && input[pos + 1].token_type == token_type_2 {
        return true;
    }

    false
}

/// Check if input[pos] is in  token_type_1 and input[pos + 1] is in token_type_2
/// return Ok(true) if both types match, Ok(false) if one of them does not
pub fn poke2vec(
    input: &[Token],
    pos: usize,
    token_type_1: Vec<TokenType>,
    token_type_2: Vec<TokenType>,
) -> bool {
    if input.len() <= pos + 1 {
        return false;
    }
    if token_type_1.contains(&input[pos].token_type)
        && token_type_2.contains(&input[pos + 1].token_type)
    {
        return true;
    }

    false
}

fn parse_bracket_arg(input: &[Token], pos: &mut usize) -> Result<NodePtr, Box<dyn Error>> {
    let mut ret = Node::new("".into(), NodeType::BracketArg);

    if !poke(input, *pos, TokenType::LeftSquareBracket) {
        panic!("Expected Left Curly Bracket!")
    }
    *pos += 1;

    let tmp = parse_paragraph(input, pos)?;

    if !poke(input, *pos, TokenType::RightSquareBracket) {
        panic!("Expected Right Curly Bracket!")
    }
    *pos += 1;

    ret.children.push(tmp);

    Ok(ret.into())
}
fn parse_brace_arg(input: &[Token], pos: &mut usize) -> Result<NodePtr, Box<dyn Error>> {
    let mut ret = Node::new("".into(), NodeType::BraceArg);

    if !poke(input, *pos, TokenType::LeftCurlyBracket) {
        panic!("Expected Left Curly Bracket!")
    }
    *pos += 1;

    let tmp = parse_paragraph(input, pos)?;

    if !poke(input, *pos, TokenType::RightCurlyBracket) {
        panic!("Expected Right Curly Bracket!")
    }
    *pos += 1;

    ret.children.push(tmp);

    Ok(ret.into())
}

// Implement the grammar
// Operation -> Word Operator Word
// Operation -> Word Operator BraceArg
// That is we are parsing things like a^b c_{aa}
// children[0] is the first part of operation, children[1] is the second part
//
// Caveat
//
// if we have something like a^bb
// then it shall be pares like a ^ b  b, (with a trailing b).
// So one word token is broken down into two.
// We can not modify input, so we pass Ok((_, true)), telling the function calling parse_operator
// th handle the trailing word
fn parse_operator(input: &[Token], pos: &mut usize) -> Result<(NodePtr, bool), Box<dyn Error>> {
    let mut ret = Node::new("".into(), NodeType::Operation);

    if !poke2vec(
        input,
        *pos,
        vec![TokenType::Word],
        vec![TokenType::Uptick, TokenType::Underline],
    ) {
        panic!("Expected Word followed by Operator!");
    }

    // The first children
    ret.children
        .push(Node::new(&input[*pos].lexeme, NodeType::Word).into());
    ret.lexeme = (&input[*pos + 1].lexeme).into();

    *pos += 2;
    if *pos >= input.len() {
        panic!("Paring operator expected Work or braced arg afte the operator!");
    }

    match input[*pos].token_type {
        TokenType::LeftCurlyBracket => {
            ret.children.push(parse_brace_arg(input, pos)?);
        }
        TokenType::Word => {
            if input[*pos].lexeme.len() == 0 {
                warn!("Expected a lexeme after opeator!");
            } else {
                ret.children
                    .push(Node::new(&input[*pos].lexeme[0..1], NodeType::Word).into());

                // The first charater is parsed as the operant.
                // Telling the caller of this function to handle the next word
                // without the first char
                return Ok((Arc::new(Mutex::new(ret)), true));
            }
        }
        _ => {}
    }

    Ok((Arc::new(Mutex::new(ret)), false))
}

fn parse_command(input: &[Token], pos: &mut usize) -> Result<NodePtr, Box<dyn Error>> {
    if !poke(input, *pos, TokenType::Command) {
        panic!("Expected Command! Internal Bug!");
    }
    let mut ret = Node::new(&input[*pos].lexeme, NodeType::Command);

    *pos += 1;

    while poke(input, *pos, TokenType::LeftSquareBracket)
        || poke(input, *pos, TokenType::LeftCurlyBracket)
    {
        if poke(input, *pos, TokenType::LeftSquareBracket) {
            ret.attach(parse_bracket_arg(input, pos)?);
        }
        if poke(input, *pos, TokenType::LeftCurlyBracket) {
            ret.attach(parse_brace_arg(input, pos)?);
        }
    }

    Ok(ret.into())
}

fn parse_math(
    input: &[Token],
    pos: &mut usize,
    end_marker: TokenType,
) -> Result<NodePtr, Box<dyn Error>> {
    let node_t: NodeType;

    match end_marker {
        TokenType::Dollar => {
            node_t = NodeType::InlineMath;
            if !poke(input, *pos, TokenType::Dollar) {
                panic!("Expected Dollar when end_marker is dollar! Internal Bug!")
            }
        }
        TokenType::DoubleDollar => {
            node_t = NodeType::DisplayMath;
            if !poke(input, *pos, TokenType::DoubleDollar) {
                panic!("Expected Double Dollar when end_marker is double dollar! Internal Bug!")
            }
        }
        _ => {
            panic!("Expected Dollar or Double Dollar! Internal Bug");
        }
    }

    *pos += 1; // we have parsed Dollar or Double Dollar
    let initial_pos = *pos;

    let mut ret = Node::new("", node_t);
    // Find the next end marker
    while *pos < input.len() && !poke(input, *pos, end_marker.clone()) {
        *pos += 1;
    }

    let mut tmp_pos = 0;
    let paragraph = parse_paragraph(&input[initial_pos..(*pos)], &mut tmp_pos)?;
    ret.attach(paragraph);

    if !poke(input, *pos, end_marker.clone()) {
        panic!("Unmatched {:?}. Found {:?}", end_marker, input[*pos]);
    } else {
        *pos += 1;
    }

    Ok(ret.into())
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
                // Check if there is operator next
                // Operators are ^ _
                if *pos + 1 < input.len() && input[*pos + 1].is_operator() {
                    let tmp = parse_operator(input, pos)?;

                    paragraph.attach(tmp.0);

                    // This means input[*pos] is a word. The first char is used
                    // as the operant.
                    // We need to handle it now, and do not count the first
                    // char twice.
                    if tmp.1 {
                        if input[*pos].lexeme.len() > 1 {
                            paragraph
                                .attach(Node::new(&input[*pos].lexeme[1..], NodeType::Word).into());
                        }
                        *pos += 1;
                    }
                } else {
                    if cur_token.lexeme.len() > 0 {
                        paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Word).into());
                    }
                    *pos += 1;
                }
            }
            TokenType::Comment => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Comment).into());
                *pos += 1;
            }
            TokenType::Backslash => {
                // This is forced, deliberate, space
                *pos += 1;
                paragraph.attach(Node::new(" ", NodeType::Word).into());
            }
            TokenType::Ampersand => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Ampersand).into());
                *pos += 1;
            }
            TokenType::Tilde => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Operation).into());
                *pos += 1;
            }
            TokenType::LeftCurlyBracket => {
                // BraceArg U
                paragraph.attach(parse_brace_arg(input, pos)?);
            }
            TokenType::LeftSquareBracket => {
                // BraceArg U
                paragraph.attach(parse_bracket_arg(input, pos)?);
            }
            TokenType::Command => {
                paragraph.attach(parse_command(input, pos)?);
            }
            TokenType::Dollar => {
                paragraph.attach(parse_math(input, pos, TokenType::Dollar)?);
            }
            TokenType::DoubleDollar => {
                paragraph.attach(parse_math(input, pos, TokenType::DoubleDollar)?);
            }
            TokenType::RightCurlyBracket | TokenType::RightSquareBracket | TokenType::Newline => {
                return Ok(ret.clone())
            }
            // TODO:
            _ => {
                panic!("Unexpected TokenType: {:?}", cur_token.token_type)
            }
        }
    }

    Ok(ret.clone())
}

#[cfg(test)]
mod test {

    #[test]
    fn parser_inline_math() {
        use crate::latex_interpreter::*;
        let input = r##"We have equation $a = b$"##;
        let tokens = scanner::scan(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_display_math() {
        use crate::latex_interpreter::*;
        let input = r##"We have equation $$a = b$$"##;
        let tokens = scanner::scan(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_command() {
        use crate::latex_interpreter::*;
        let input = r##"\a{aaa}[abb]{asb}"##;
        let tokens = scanner::scan(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_try() {
        use crate::latex_interpreter::*;
        let input = r##"Hello!     Junlu&!
    [aaa a]
abc^def

e^{i p} + 1 = 0
Another paragraph!
"##;
        let tokens = scanner::scan(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }
}
