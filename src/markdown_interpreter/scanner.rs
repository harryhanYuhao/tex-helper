/// A custom scanner for Markdown
///
/// Scan input a &str and output a Vec of Tokens. 
/// Tokens, for the most parts, are scanned in the obvious way: all speical
/// characters (including newline and space) have their own token types. 
/// 
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TokenType {
    // Reserved Characters
    Dollar,            // $
    DoubleDollar,      // double dollar must be consective
    Uptick,            // ^
    Ampersand,         // &
    Underline,         // _
    LeftCurlyBracket,  // {
    RightCurlyBracket, // }
                       
    Star,              // *
    Slash,         // -
    Section(u8),   // # or ## 
    Indent(u8),    //

    // Backslash is almost never used alone. When appear by itself, it create space in text mode
    Backslash, // \
    Tilde,     // ~

    DoubleBackslash, // \\
    Command,
    LeftSquareBracket,  // [
    RightSquareBracket, // ]

    SlashOpenBracket,  // \[
    SlashCloseBracket, // \]

    Word, // Text does not contains any space

    // Escaped Characters can not be simply treated as Text
    // Some of them have special functionalities
    EscapedChar,
    Space,
    // The job of making two newlines into a paragraph is left to the parser
    Newline,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token { token_type, lexeme }
    }
}

pub fn scan(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let length = chars.len();

    let mut ret: Vec<Token> = Vec::new();
    let mut i = 0;

    // Note we have an i+=1 at the end of the loop
    // so in match, i shall only be incremented with the extra space
    while i < length {
        match chars[i] {
            // '#' => {
            //     ret.push(Token::new(TokenType::Hash, "#".into()));
            // }
            '*' => {
                ret.push(Token::new(TokenType::Star, "*".into()));
            }
            '$' => {
                if i + 1 < length && chars[i + 1] == '$' {
                    ret.push(Token::new(TokenType::DoubleDollar, "$$".into()));
                    i += 1; // Skip the next '$'
                } else {
                    ret.push(Token::new(TokenType::Dollar, "$".into()));
                }
            }
            '%' => {
                // check doc/latex_grammar/1_overview.md#Comments
                // commands are ignored until the end of an line
                let end_of_line = index_to_end_of_cur_line(&chars, i);
                ret.push(Token::new(TokenType::Space, String::new()));

                if is_beginning_of_line(&chars, i) || end_of_line == chars.len() - 1 {
                    // if the comment is at the beginning of a line, ignore the
                    // last new line character
                    //
                    // Note the end_of_line function return the index of the
                    // last character index is at the last line.
                    i = end_of_line;
                } else {
                    i = end_of_line - 1;
                }
            }
            '^' => {
                ret.push(Token::new(TokenType::Uptick, "^".into()));
            }
            '&' => {
                ret.push(Token::new(TokenType::Ampersand, "&".into()));
            }
            '_' => {
                ret.push(Token::new(TokenType::Underline, "_".into()));
            }
            '{' => {
                ret.push(Token::new(TokenType::LeftCurlyBracket, "{".into()));
            }
            '}' => {
                ret.push(Token::new(TokenType::RightCurlyBracket, "}".into()));
            }
            '\\' => {
                if i + 1 >= length {
                    ret.push(Token::new(TokenType::Backslash, "\\".into()));
                } else if chars[i + 1] == '\\' {
                    ret.push(Token::new(TokenType::DoubleBackslash, String::new()));
                    i += 1;
                } else if chars[i + 1] == '#'
                    || chars[i + 1] == '$'
                    || chars[i + 1] == '%'
                    || chars[i + 1] == '^'
                    || chars[i + 1] == '&'
                    || chars[i + 1] == '_'
                    || chars[i + 1] == '{'
                    || chars[i + 1] == '}'
                    || chars[i + 1] == '~'
                    || chars[i + 1] == ' '
                {
                    ret.push(Token::new(TokenType::EscapedChar, chars[i + 1].into()));
                    i += 1;
                } else if chars[i + 1] == '\n' {
                    ret.push(Token::new(TokenType::Backslash, "\\".into()));
                    // note we do not increase i+1 here.
                } else if chars[i + 1] == '[' {
                    ret.push(Token::new(TokenType::SlashOpenBracket, "\\[".into()));
                    i += 1;
                } else if chars[i + 1] == ']' {
                    ret.push(Token::new(TokenType::SlashCloseBracket, "\\]".into()));
                    i += 1;
                } else if chars[i + 1].is_alphabetic() {
                    let start = i + 1;
                    while i + 1 < length && chars[i + 1].is_alphabetic() {
                        i += 1
                    }
                    ret.push(Token::new(
                        TokenType::Command,
                        chars[start..=i].iter().collect(),
                    ));
                }
            }
            '~' => {
                ret.push(Token::new(TokenType::Tilde, "~".into()));
            }
            '[' => {
                ret.push(Token::new(TokenType::LeftSquareBracket, "[".into()));
            }
            ']' => {
                ret.push(Token::new(TokenType::RightSquareBracket, "]".into()));
            }
            ' ' | '\t' => {
                if is_beginning_of_line(&chars, i) {
                    //
                } else {
                    while i + 1 < length && (chars[i + 1] == ' ' || chars[i + 1] == '\t') {
                        i += 1;
                    }
                    ret.push(Token::new(TokenType::Space, String::new()));
                }
            }
            '\n' => {
                ret.push(Token::new(TokenType::Newline, "\n".into()));
            }
            _ => {
                // Scan text until next reserved character or whitespace
                let start = i;
                while i + 1 < length
                    && ![
                        '#', '$', '%', '^', '&', '_', '{', '}', '\\', '~', '[', ']', ' ',
                    ]
                    .contains(&chars[i + 1])
                    && !chars[i + 1].is_whitespace()
                {
                    i += 1;
                }
                ret.push(Token::new(
                    TokenType::Word,
                    chars[start..=i].iter().collect::<String>(),
                ));
            }
        }

        i += 1;
    }
    ret
}

/// return true if index = 0, or there is only spaces between source[index] and the previous newline
/// or the 0th index
///
/// In particular, as latex ignore the beginning spaces of a line
/// the first non-space character and all space before it are all considered
/// as the beg
///
/// Eg:
///# aaa {
///#    ^
///#     is not beginning of group
///# arma virumque cano        {Trioae}
///#                           ^
///#                          is beginning of group
/// Will panic if index is not valid, that is, index > source.len()
fn is_beginning_of_group(source: &[char], index: usize) -> bool {
    if index >= source.len() {
        panic!("Index >= source.len() in function is_beginning_of_line. Program internal bug.");
    }
    let mut i = index;
    while i > 0 && (source[i - 1] == ' ' || source[i - 1] == '\t') {
        i -= 1;
    }

    if i == 0 || source[i - 1] == '{' {
        return true;
    }
    false
}
/// return true if index = 0, or there is only spaces between source[index] and the previous newline
/// or the 0th index
///
/// In particular, as latex ignore the beginning spaces of a line
/// the first non-space character and all space before it are all considered
/// as the beg
///
/// Eg:
///# aaa\n
///#     ^
///#      is not beginning of line
///# arma virumque cano \n     Trioae
///#                           ^
///#                           is beginning of line
/// Will panic if index is not valid, that is, index > source.len()
fn is_beginning_of_line(source: &[char], index: usize) -> bool {
    if index >= source.len() {
        panic!("Index >= source.len() in function is_beginning_of_line. Program internal bug.");
    }
    let mut i = index;
    while i > 0 && (source[i - 1] == ' ' || source[i - 1] == '\t') {
        i -= 1;
    }

    if i == 0 || source[i - 1] == '\n' {
        return true;
    }
    false
}

/// return the index of the end of the current line, including the newline character
/// If the current line is the last line, return the last index
///
/// EG
/// / aaa\n
/// /    ^ //return 3
/// / \n\n
/// / ^   //return 0
/// aaaa
///    ^ //return 3
/// Will panic if index is not valid, that is, index > source.len()
fn index_to_end_of_cur_line(source: &[char], index: usize) -> usize {
    if index >= source.len() {
        panic!("Index >= source.len() in function is_beginning_of_line. Program internal bug.");
    }

    let mut i = index;
    while i < source.len() && source[i] != '\n' {
        i += 1;
    }
    // if we are at the end of the source, just return the last index
    if i == source.len() {
        return i - 1;
    }
    i
}
