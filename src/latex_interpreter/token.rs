//! Valid tokens for latex are defined here.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub row: usize, // row (line) number in the source file, starting from 0
    pub col: usize, // column number in the source file, starting from 0
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    // Reserved Characters
    Hash,         // #
    Dollar,       // $
    DoubleDollar, // double dollar must be consective
    Ampersand,    // &

    // These three are classified as operator
    Tilde,     // ~
    Uptick,    // ^
    Underline, // _

    LeftCurlyBracket,  // {
    RightCurlyBracket, // }

    // Backslash is almost never used alone. When appear by itself, it create space in text mode
    Backslash, // \

    DoubleBackslash, // \\

    Command,

    LeftSquareBracket,  // [
    RightSquareBracket, // ]

    SlashOpenBracket,  // \[
    SlashCloseBracket, // \]

    // Text that does not contains any space
    // the basic unit for text is word instead of sentence, because that are other
    Word,

    // word-like units, like comment and inline math
    // Can not just ignore the comment, as we are working on a formatter
    Comment,

    // Escaped Characters can not be simply treated as Text
    // Some of them have special functionalities
    EscapedChar,

    // Two or more consecutive newlines, which marks a new parragraph
    // A sinle newline is ignored (just like space)
    NewParagraph,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        row: usize,
        col: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            row,
            col,
        }
    }

    pub fn to_string_from_vec(tokens: &[Token]) -> String {
        let mut ret = String::new();

        for i in tokens {
            ret.push_str(&format!("{}", i));
        }
        ret
    }

    pub fn is_operator(&self) -> bool {
        self.token_type == TokenType::Uptick
            || self.token_type == TokenType::Underline
    }

    pub fn is_begin_envr(&self) -> bool {
        self.token_type == TokenType::Command && self.lexeme == "begin"
    }

    pub fn is_end_envr(&self) -> bool {
        self.token_type == TokenType::Command && self.lexeme == "end"
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut ret = format!("{:?}({:?})", self.token_type, self.lexeme);
        if self.token_type == TokenType::NewParagraph {
            ret.push_str("\n");
        } else {
            ret.push_str(" ");
        }
        write!(f, "{}", ret)
    }
}
