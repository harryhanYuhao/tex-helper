//! This is a naive LL(2) recursive parser for Latex
//! The parser parse vec of token types in AST(abstract syntax tree)
//! The parser will delete and arrange tokens, but will not modify the lexeme of any token
//!
//! This is the parse grammar
//! Note * denotes 0 or more, + denotes 1 or more
//!
//! Passage -> Passage Paragraph \n\n+ (two or more consective line breaks)
//!
//! Paragraph -> Paragraph E
//!
//! E -> Word
//! E -> Commant
//! E -> LoneCommand   // Commands without args
//! E -> SPACE* ( space are simple ignored)
//!
//! Space -> \n  // a single line break is a space
//! Space -> ' ' | '\t'+  // one or more consecutive space (or tabs) is considered as a single space
//!
//!
//! E -> Operation
//! Operation -> Word Operator Word
//! Operation -> Word Operator BraceArg
//! IMPORTANT: not parsing of operation has a complication that ab^12 shall be parsed as a b^1 2.
//! This is taken care of in parse_operator function.
//! The description of this grammar however, can not be expressed in BNF
//!
//! E -> CommandWithArg
//! CommandWithArg -> LoneCommand (BraceArg | BracketArg)+
//! BraceArg -> {Paragraph}
//! BracketArg -> [Paragraph]

use super::ast::{Node, NodePtr, NodeType};
use super::scanner::{scan_str, Token, TokenType};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};


/// This is the main function of this file
pub fn parse(input: &[Token]) -> Result<NodePtr, Box<dyn Error>> {
    let mut pos: usize = 0;
    Ok(parse_passage(input, &mut pos)?)
}

pub fn parse_passage(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
    let root_ptr = Node::empty_passage_ptr();

    let mut root = root_ptr.lock().unwrap();
    let mut prev_pos = *pos; // For debug purpose

    while *pos < input.len() {
        let paragraph = parse_paragraph(input, pos)?;

        if poke(input, *pos, TokenType::Newline) {
            root.attach(paragraph);
            *pos += 1;
        } else {
            root.attach(paragraph);
            break;
        }

        // For debug purpose
        if prev_pos == *pos {
            panic!("parse in infinite loop!")
        }
        prev_pos = *pos;
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
    if input[pos].token_type == token_type_1
        && input[pos + 1].token_type == token_type_2
    {
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

fn parse_square_bracket_arg(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
    let mut ret = Node::new("".into(), NodeType::SquareBracketArg);

    if !poke(input, *pos, TokenType::LeftSquareBracket) {
        panic!("Expected Left Curly Bracket! Found {:?}", input[*pos]);
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

fn parse_curly_bracket_arg(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
    let mut ret = Node::new("".into(), NodeType::CurlyBracketArg);

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
// CAVEAT!!!
//
// a^bb shall be parsed as a^b  b, (with a trailing b).
// ab^2, shall be parsed as a b^2
//
// So one word token may be broken down into two.
// We can not modify input, so, instead, we return a vec of NodePtr, all of which shall be pushed
// into the callers' managed node's children
// Of course, pos is incremented according to number of token parsed by this function
fn parse_operator(
    input: &[Token],
    pos: &mut usize,
) -> Result<Vec<NodePtr>, Box<dyn Error>> {
    let mut ret: Vec<NodePtr> = vec![];
    let mut op_root = Node::new("".into(), NodeType::Operation);

    if !poke2vec(
        input,
        *pos,
        vec![TokenType::Word],
        vec![TokenType::Uptick, TokenType::Underline],
    ) {
        panic!("Expected Word followed by Operator!");
    }

    // Now, we have
    // input = WORD   OP          ...
    //         *pos   *pos + 1
    op_root.lexeme = (&input[*pos + 1].lexeme).into();

    // Check the lexeme of Word. as ab^2 shall be considered as a b^2
    // In latex, a lone ^2 is valid
    if input[*pos].lexeme.len() <= 1 {
        op_root
            .children
            .push(Node::new(&input[*pos].lexeme, NodeType::Word).into());
    } else {
        // we are at the case of ab^2. Create new word a, append to ret. Create a new
        // word with lexeme b and append to the child of op_root, as the first
        // arg of operation
        let pre_word_len = input[*pos].lexeme.len();
        let pre_word = Node::new(
            &input[*pos].lexeme[0..(pre_word_len - 1)],
            NodeType::Word,
        );

        ret.push(pre_word.into());

        op_root.children.push(
            Node::new(
                &input[*pos].lexeme[(pre_word_len - 1)..],
                NodeType::Word,
            )
            .into(),
        );
    }

    *pos += 2;

    if *pos >= input.len() {
        panic!(
            "Paring operator expected Work or braced arg afte the operator!"
        );
    }

    match input[*pos].token_type {
        TokenType::LeftCurlyBracket => {
            op_root.children.push(parse_curly_bracket_arg(input, pos)?);
            ret.push(op_root.into());
        }
        TokenType::Word => {
            let wordlen = input[*pos].lexeme.len();

            match wordlen {
                0 => {
                    // ^2 is valid, 2^ is not
                    warn!("Expected a lexeme after opeator!");
                }
                1 => {
                    op_root.children.push(
                        Node::new(&input[*pos].lexeme, NodeType::Word).into(),
                    );
                    ret.push(op_root.into());
                }
                _ => {
                    // we are in the case a^23, which shall be parsed as a^2 3
                    op_root.children.push(
                        Node::new(&input[*pos].lexeme[0..1], NodeType::Word)
                            .into(),
                    );
                    ret.push(op_root.into());
                    let post_word =
                        Node::new(&input[*pos].lexeme[1..], NodeType::Word);
                    ret.push(post_word.into());
                }
            }
            *pos += 1;
        }
        _ => {
            panic!(
                "Unexpected Token: {:?}. Expected Word or Braced Arg after operator.",
                input[*pos].token_type
            );
        }
    }

    Ok(ret)
}

fn parse_command(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
    if !poke(input, *pos, TokenType::Command) {
        panic!("Expected Command! Internal Bug!");
    }
    let mut ret = Node::new(&input[*pos].lexeme, NodeType::Command);

    *pos += 1;

    while poke(input, *pos, TokenType::LeftSquareBracket)
        || poke(input, *pos, TokenType::LeftCurlyBracket)
    {
        if poke(input, *pos, TokenType::LeftSquareBracket) {
            ret.attach(parse_square_bracket_arg(input, pos)?);
        }
        if poke(input, *pos, TokenType::LeftCurlyBracket) {
            ret.attach(parse_curly_bracket_arg(input, pos)?);
        }
    }

    Ok(ret.into())
}

/// Parse paragraph calls parse_math when it sees $ or $$
/// since we are parsing recursively, we need to know the where end marker is
/// Here we adopted a naive approach.
fn parse_math(
    input: &[Token],
    pos: &mut usize,
    end_marker: TokenType,
) -> Result<NodePtr, Box<dyn Error>> {
    let node_t: NodeType;

    // Error handling
    match end_marker {
        TokenType::Dollar => {
            node_t = NodeType::InlineMath;
            if !poke(input, *pos, TokenType::Dollar) {
                panic!(
                    "Expected Dollar when end_marker is dollar! Internal Bug!"
                )
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
    let mut ret = Node::new("", node_t);

    *pos += 1; // we have parsed Dollar or Double Dollar
    let initial_pos = *pos;

    // Find the next end marker
    while *pos < input.len() && !poke(input, *pos, end_marker.clone()) {
        *pos += 1;
    }

    // We have two cases here
    // 1. end marker is found
    // $ ..... $ ..
    //         ^ (*pos is here)
    // 2. END is reached without finding end marker: error handling
    // $ ..... $ EOF
    //           ^ (*pos is here)
    // TODO: error handling
    if *pos == input.len() {
        panic!("Unmatched {:?}", end_marker.clone());
    }

    let mut tmp_pos = 0;
    let paragraph = parse_paragraph(&input[initial_pos..(*pos)], &mut tmp_pos)?;

    ret.attach(paragraph);

    *pos += 1;

    Ok(ret.into())
}

fn parse_slash_open_bracket(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
    let mut ret = Node::new("", NodeType::DisplayMath);

    if !poke(input, *pos, TokenType::SlashOpenBracket) {
        panic!("Internal Error! Expected SlashOpenBracket!")
    }

    *pos += 1;
    ret.children.push(parse_paragraph(input, pos)?);

    // TODO: ERROR HANDLING
    if !poke(input, *pos, TokenType::SlashCloseBracket) {
        panic!("Internal Error! Expected SlashCloseBracket!")
    }
    *pos += 1;

    Ok(ret.into())
}

fn parse_envr(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
    if !poke(input, *pos, TokenType::Command) || !input[*pos].is_begin_envr() {
        panic!("Internal Error! Expected begin environment!")
    }
    // The environments are like
    // \begin{envr_name}
    // \end{envr_name}

    *pos += 1;

    let envr_arg = parse_curly_bracket_arg(input, pos)?;
    let envr_name: String =
        Node::get_string_content_recur_nodeptr(envr_arg.clone());

    let mut ret = Node::new(&envr_name, NodeType::Envr);

    ret.children.push(parse_passage(input, pos)?);

    // TODO: ERROR HANDLING
    if !poke(input, *pos, TokenType::Command) || !input[*pos].is_end_envr() {
        panic!("Internal Error! Expected End environment!")
    }

    *pos += 1;
    // we are now at
    // \end{envr_name}
    //      ^
    // still need to parse the end brace arg

    let envr_end_arg = parse_curly_bracket_arg(input, pos)?;
    let envr_end_name: String =
        Node::get_string_content_recur_nodeptr(envr_end_arg.clone());

    if envr_end_name != envr_name {
        panic!(
            "Unmatched environment! Expected {}, found {}",
            envr_name, envr_end_name
        );
    }

    Ok(ret.into())
}
// This is the main parse logic, as the whole latex file is a paragraph
// We are implementing a simple LL(1) recursive parser
fn parse_paragraph(
    input: &[Token],
    pos: &mut usize,
) -> Result<NodePtr, Box<dyn Error>> {
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
                    for i in tmp.iter() {
                        paragraph.attach(i.clone());
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
            TokenType::DoubleBackslash => {
                // Line break but not paragraph break
                *pos += 1;
                paragraph.attach(Node::new("\n", NodeType::Word).into());
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
                paragraph.attach(parse_curly_bracket_arg(input, pos)?);
            }
            TokenType::LeftSquareBracket => {
                // BraceArg U
                paragraph.attach(parse_square_bracket_arg(input, pos)?);
            }
            TokenType::Dollar => {
                paragraph.attach(parse_math(input, pos, TokenType::Dollar)?);
            }
            TokenType::DoubleDollar => {
                paragraph.attach(parse_math(input, pos, TokenType::DoubleDollar)?);
            }
            TokenType::SlashOpenBracket => {
                paragraph.attach(parse_slash_open_bracket(input, pos)?);
            }
            TokenType::Command => {
                // command could be environment
                if input[*pos].is_begin_envr() {
                    paragraph.attach(parse_envr(input, pos)?);
                } else if input[*pos].is_end_envr() {
                    return Ok(ret.clone());
                } else {
                    paragraph.attach(parse_command(input, pos)?);
                }
            }
            TokenType::RightCurlyBracket  // end of brace args 
            | TokenType::RightSquareBracket  // end of bracket args 
            | TokenType::SlashCloseBracket  // end of display math
            | TokenType::Newline => return Ok(ret.clone()),
            _ => {
                // TODO: error handling
                panic!("Unexpected TokenType: {:?}", cur_token.token_type)
            }
        }
    }

    Ok(ret.clone())
}

#[cfg(test)]
mod test {

    use crate::latex_interpreter::*;
    #[test]
    fn string_content_recur() {
        let input = r##"aaabbb"##;
        let tokens = scanner::scan_str(input);
        let ast = parser::parse(&tokens).unwrap();
        println!("{}", ast.lock().unwrap());
        let ast = ast.lock().unwrap();
        assert_eq!(input, ast.get_string_content_recur());
    }

    #[test]
    fn parser_envr() {
        let input = r##"\begin{document}
This is document
\begin{eq}
e=mc^2
    \begin{bmatrix}
     % We have an empty matrix
    \end{bmatrix}
\end{eq}
Hope there is success!
\end{document}"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_slash_open_bracket() {
        let input = r##"\[e = mc^2\]"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_operator() {
        let input = r##"e^{aaa}"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_inline_math() {
        let input = r##"We have equation $e=mc^2$"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_display_math() {
        let input = r##"We have equation $$a = b$$"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_command() {
        let input = r##"\a{aaa}[abb]{asb}"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_try() {
        let input = r##"Hello!     Junlu&!
    [aaa a]
abc^def

e^{i p} + 1 = 0
Another paragraph!
"##;
        let tokens = scanner::scan_str(input);
        println!("Tokens:\n{}", scanner::Token::to_string_from_vec(&tokens));
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }

    #[test]
    fn parser_comprehensive_test() {
        let input = r##"\documentclass[12pt, a4paper]{article}
\usepackage{blindtext, titlesec, amsthm, thmtools, amsmath, amsfonts, scalerel, amssymb, graphicx, titlesec, xcolor, multicol, hyperref}
\usepackage[utf8]{inputenc}
\hypersetup{colorlinks,linkcolor={red!40!black},citecolor={blue!50!black},urlcolor={blue!80!black}}
\newtheorem{theorem}{Theorema}[subsection]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{corollary}[theorem]{Corollarium}
\newtheorem{hypothesis}{Coniectura}
\theoremstyle{definition}
\newtheorem{definition}{Definitio}[section]
\theoremstyle{remark}
\newtheorem{remark}{Observatio}[section]
\newtheorem{example}{Exampli Gratia}[section]
\newcommand{\bb}[1]{\mathbb{1}}
\renewcommand\qedsymbol{Q.E.D.}
\title{title}
\author{aaa}
\date{\today}
\begin{document}
\maketitle
%\tableofcontents

\end{document}
"##;
        let tokens = scanner::scan_str(input);
        let ast = parser::parse(&tokens).unwrap();

        println!("{}", ast.lock().unwrap());
    }
}
