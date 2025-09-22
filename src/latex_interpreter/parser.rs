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
        root.attach(parse_paragraph(input, &mut pos)?);

        if pos < input.len() && poke2(input, pos, TokenType::Newline, TokenType::Newline) {
            parse_consecutive_line_breaks(input, &mut pos);
        }

        // For debug purpose
        if prev_pos == pos {
            panic!("parse in infinite loop!")
        }
        prev_pos = pos;
    }

    Ok(root_ptr.clone())
}

/// Check if input[pos] == token_type_1, return Ok(true) if it is, Ok(false) if it is not
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

/// increment pos for the number of consecutive newline token
/// If reaching the end of the input, or meeting tokens of another tokentype, simple returns
///
/// This function shall only be called when input[*pos] newline, otherwise will panic
///
/// eg:         
/// If we have input:
/// input = WORD NEWLINE
/// and *pos = 1
/// PANIC!!!!
///
/// If we have input:
/// input = WORD NEWLINE NEWLINE NEWLINE WORD ...
/// and *pos = 1
/// After returning, *pos = 4
fn parse_consecutive_line_breaks(input: &[Token], pos: &mut usize) {
    if *pos + 1 >= input.len() || !poke2(input, *pos, TokenType::Newline, TokenType::Newline) {
        panic!("Expected Two consecutive newlines!")
    }
    while *pos < input.len() && input[*pos].token_type == TokenType::Newline {
        *pos += 1;
    }
}

/// increment pos for the number of consecutive space or newline token, as a single new line is considered as space
/// If reaching the end of the input, meeting two consective newline tokens, or
/// meeting tokens of another tokentype, simple returns
///
/// This function shall only be called when input[*pos] is space or newline, otherwise will panic
///
/// eg:         
/// If we have input:
/// input = WORD SPACE NEWLINE SPACE WORD ...
/// and *pos = 1
/// After returning, *pos is modified to 4
///
/// If we have input:
/// input = WORD SPACE NEWLINE NEWLINE WORD ...
/// and *pos = 0
/// PANIC!!!! (input[0] is not SPACE or NEWLINE)
///
/// If we have input:
/// input = WORD SPACE NEWLINE NEWLINE WORD ...
/// and *pos = 1
/// After returning, *pos = 2 (Two consecutive newlines is a paragraph break, not)
fn parse_space(input: &[Token], pos: &mut usize) {
    if *pos >= input.len() {
        panic!("*pos >= input.len()!!!");
    }
    if input[*pos].token_type != TokenType::Space && input[*pos].token_type != TokenType::Newline {
        panic!(
            "Expected: Space or NewLine token, Found: {:?}",
            input[*pos].token_type
        );
    }

    while *pos < input.len()
        && (input[*pos].token_type == TokenType::Space
            || input[*pos].token_type == TokenType::Newline)
    {
        // In the case of two consecutive newlines, just return
        if input[*pos].token_type == TokenType::Newline {
            if *pos + 1 < input.len() && input[*pos + 1].token_type == TokenType::Newline {
                return;
            }
        }
        *pos += 1;
    }
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

// This is the main parse logic, as the whole latex file is a paragraph
// We are implementing a simple LL(1) recursive parser
fn parse_paragraph(input: &[Token], pos: &mut usize) -> Result<NodePtr, Box<dyn Error>> {
    let ret: Arc<Mutex<Node>> = Node::empty_paragraph_ptr();
    let mut paragraph = ret.lock().unwrap();

    while *pos < input.len() {
        let cur_token = &input[*pos];
        match cur_token.token_type {
            TokenType::Word => {
                let lexeme = &cur_token.lexeme.clone();

                // Check if there is operator next
                // Operators are ^ _
                // TODO:: Implement Word SPACE* OPERATOR
                // TODO: CHECK NEXT NONE SPACE TOKEN
                if *pos+1 < input.len() && input[*pos+1].is_operator() {
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
            TokenType::Space => {
                parse_space(input, pos);
                // between words token, we assume there is a space
                // The space token is for manually created space like `\ `
                // NOTE:: maybe we can simply ignore space token type in the
                // scanner
            }
            TokenType::Newline => {
                // In case of two consecutive newline, return ret
                // and let the parse function to handle
                if poke2(input, *pos, TokenType::Newline, TokenType::Newline) {
                    return Ok(ret.clone());
                }
                parse_space(input, pos);
            }
            TokenType::Backslash => {
                // This is forced, deliberate, space
                *pos += 1;
                paragraph.attach(Node::new(" ", NodeType::Space).into());
            }
            TokenType::Ampersand => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Ampersand).into());
                *pos += 1;
            }
            TokenType::Tilde => {
                paragraph.attach(Node::new(&cur_token.lexeme, NodeType::Operation).into());
                *pos += 1;
            }
            TokenType::RightCurlyBracket
            | TokenType::Dollar
            | TokenType::RightSquareBracket
            | TokenType::DoubleDollar => return Ok(ret.clone()),

            TokenType::LeftCurlyBracket => {
                // BraceArg U
                paragraph.attach(parse_brace_arg(input, pos)?);
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
    fn parser_try() {
        use crate::latex_interpreter::*;
        let input = r##"Hello!     Junlu&! \
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
