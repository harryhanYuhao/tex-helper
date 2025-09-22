use std::fmt::{self, Display, Formatter};

/// A custom scanner for LaTex
///
/// This files defines the struct Token and the scan function.
///
/// Scan input a &str and output a Vec of Tokens.
/// Tokens, for the most parts, are scanned in the obvious way: all speical
/// characters (including newline and space) have their own token types.
///
/// Here are the description of the implementation
/// 1, Each token consists of a TokenType and a lexeme.
/// The lexeme is ignored for certain TokenTypes, such as Space, Newline,
/// 1. Multiple spaces are treated as one space token, except when they are at the beginning of a
///     line, in which case they are ignored.
/// 1. Comments are treated as a single space token, except when they occupies the whole line, in
///    which case the newline character at the end of the line is also ignored. check
///     1. This means sometime there are several consecutive space tokens. The handle of which is
///        left to the parser.
///    doc/latex_grammar/1_overview.md#Comments for more details.
/// 1. The basic token for Text is Word
/// which is scanned until the next special character or whitespace.
/// So `apple banana orange` will be scanned into 5 tokens: Word(apple), Space, Word(b), Space, Word(orange)
/// 1. Commands are scanned into command tokens, the beginning backslash is not in the lexeme.
/// 1. Escaped characters are into EscapedChar, the backslash is not in the lexeme.

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
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

    Word, // Text that does not contains any space
    // the basic unit for text is word instead of sentence, because that are other
    // word-like units, like comment and inline math
    Comment, // Can not just ignore the comment, as we are working on a formatter

    // Escaped Characters can not be simply treated as Text
    // Some of them have special functionalities
    EscapedChar,
    
    Newline,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token { token_type, lexeme }
    }

    pub fn to_string_from_vec(tokens: &[Token]) -> String {
        let mut ret = String::new();

        for i in tokens {
            ret.push_str(&format!("{}", i));
        }
        ret
    }

    pub fn is_operator(&self) -> bool {
        self.token_type == TokenType::Uptick || self.token_type == TokenType::Underline
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut ret = format!("{:?}({:?})", self.token_type, self.lexeme);
        if self.token_type == TokenType::Newline {
            ret.push_str("\n");
        } else {
            ret.push_str(" ");
        }
        write!(f, "{}", ret)
    }
}

/// This is the major function of this file.
///
/// Input: A string representing latex code read from Latex file
/// Output: A vector of Tokens
///
/// This function implements a naive regex algorithm.
/// TODO: describe formally the algorithm, and the expected output
pub fn scan(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let length = chars.len();

    let mut ret: Vec<Token> = Vec::new();
    let mut i = 0;

    // Note we have an i+=1 at the end of the loop
    // so in match, i shall only be incremented with the extra space
    while i < length {
        match chars[i] {
            '#' => {
                ret.push(Token::new(TokenType::Hash, "#".into()));
            }
            '$' => {
                if i + 1 < length && chars[i + 1] == '$' {
                    ret.push(Token::new(TokenType::DoubleDollar, "$$".into()));
                    i += 1; // Skip the next '$'
                } else {
                    ret.push(Token::new(TokenType::Dollar, "$".into()));
                }
            }
            // As we are working on a formatter, we can not just ignore the comments
            // check doc/latex_grammar/1_overview.md#Comments  for behaviour of
            // comments in latex
            '%' => {
                let end_of_line = index_to_end_of_cur_line(&chars, i);

                // index_to_end_of_cur_line returns the index of next \n char
                // marking the end of current line
                // however, if the current line is the end of the document and does
                // not contain a \n, it returns the index of last character of the
                // document
                if end_of_line == chars.len() - 1 && chars[end_of_line] != '\n' {
                    ret.push(Token::new(
                        TokenType::Comment,
                        chars[i + 1..=end_of_line].iter().collect(),
                    ));
                    i = end_of_line;
                } else {
                    ret.push(Token::new(
                        TokenType::Comment,
                        chars[i + 1..end_of_line].iter().collect(),
                    ));
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
                    // ret.push(Token::new(TokenType::Space, String::new()));
                }
            }
            '\n' => {
                let mut newline_count = 1;
                while i + 1 < length && (chars[i + 1] == ' ' || chars[i+1] == '\t' || chars[i + 1] == '\n'){
                    if chars[i+1] == '\n' {
                        newline_count += 1;
                    }
                    i += 1;
                }
                if newline_count >= 2{
                    ret.push(Token::new(TokenType::Newline, "\n".into()));
                }
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
        } // end of match

        i += 1;
    } // end of loop
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

/// return the index of the \n char marking the end of the current line
/// If the current line is the last line in the document,it
/// return the last index
///
/// EG
/// $ aaa\n
/// $    ^ //return 3
/// $ \n\n
/// $ ^   //return 0
/// $ aaaa (End of Document)
/// $    ^ //return 3
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

#[cfg(test)]
mod test_scan {
    use super::*;

    /// Aux function for test
    fn compare_expected_and_tokens(expected: Vec<(TokenType, String)>, tokens: Vec<Token>) {
        assert_eq!(expected.len(), tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            if token.token_type != expected[i].0 {
                panic!(
                    "Token type mismatch at index {}: expected {:?}, got {:?}",
                    i, expected[i].0, token.token_type
                );
            }
            if token.lexeme != expected[i].1 {
                panic!(
                    "Token lexeme mismatch at index {}: expected {:?}, got {:?}",
                    i, expected[i].1, token.lexeme
                );
            }
        }
    }

    #[test]
    fn test_is_beginning_of_line() {
        let s: Vec<char> = "012\n  6\n89".chars().collect();
        assert!(is_beginning_of_line(&s, 0));
        assert!(is_beginning_of_line(&s, 5));
        assert!(is_beginning_of_line(&s, 6));
        assert!(is_beginning_of_line(&s, 8));

        assert!(!is_beginning_of_line(&s, 2));
        assert!(!is_beginning_of_line(&s, 9));
    }

    #[test]
    fn test_index_to_end_of_line() {
        let s: Vec<char> = "012\n  6\n89".chars().collect();
        assert_eq!(index_to_end_of_cur_line(&s, 0), 3);
        assert_eq!(index_to_end_of_cur_line(&s, 3), 3);
        assert_eq!(index_to_end_of_cur_line(&s, 6), 7);
        assert_eq!(index_to_end_of_cur_line(&s, 8), 9);
    }

    #[test]
    fn test_fnscan_space() {
        let tokens = scan("  a bc  d ");
        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Word, "a".into()),
            (TokenType::Word, "bc".into()),
            (TokenType::Word, "d".into()),
        ];
        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_fnscan_newline() {
        let tokens = scan("a\nb");
        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Word, "a".into()),
            (TokenType::Word, "b".into()),
        ];
        compare_expected_and_tokens(expected, tokens);

        let tokens = scan(
            r##"a % A comment
%
aaa"##,
        );
        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Word, "a".into()),
            (TokenType::Comment, " A comment".into()),
            (TokenType::Comment, "".into()),
            (TokenType::Word, "aaa".into()),
        ];
        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_slash_bracket() {
        let tokens = scan(r"\[ \]");
        let expected = vec![
            (TokenType::SlashOpenBracket, String::from("\\[")),
            (TokenType::SlashCloseBracket, String::from("\\]")),
        ];
        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_short_math_mode() {
        let tokens = scan(r"$E=mc^2$");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Dollar);
        assert_eq!(tokens[1].token_type, TokenType::Word);
        assert_eq!(tokens[1].lexeme, "E=mc");
        assert_eq!(tokens[2].token_type, TokenType::Uptick);
        assert_eq!(tokens[3].token_type, TokenType::Word);
        assert_eq!(tokens[3].lexeme, "2");
        assert_eq!(tokens[4].token_type, TokenType::Dollar);
    }

    #[test]
    fn test_long_text_mode() {
        let tokens = scan(r"$$E=mc^2$$");
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::DoubleDollar);
        assert_eq!(tokens[1].token_type, TokenType::Word);
        assert_eq!(tokens[1].lexeme, "E=mc");
        assert_eq!(tokens[2].token_type, TokenType::Uptick);
        assert_eq!(tokens[3].token_type, TokenType::Word);
        assert_eq!(tokens[3].lexeme, "2");
        assert_eq!(tokens[4].token_type, TokenType::DoubleDollar);
    }

    #[test]
    fn test_short_text() {
        let tokens = scan("arma virumque cano , ");

        let expected: Vec<(TokenType, String)>= vec![
            (TokenType::Word, "arma".into()),
            (TokenType::Word, "virumque".into()),
            (TokenType::Word, "cano".into()),
            (TokenType::Word, ",".into()),
        ];
        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_long_text() {
        let tokens = scan(
            r##"arma virumque cano, Troiae qui primus ab oris 
Italiam, fato profugus, Laviniaque venit 
litora, multum ille et terris iactatus et alto 
vi superum saevae memorem Iunonis ob iram"##,
        );

        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Word, "arma".into()),
            (TokenType::Word, "virumque".into()),
            (TokenType::Word, "cano,".into()),
            (TokenType::Word, "Troiae".into()),
            (TokenType::Word, "qui".into()),
            (TokenType::Word, "primus".into()),
            (TokenType::Word, "ab".into()),
            (TokenType::Word, "oris".into()),
            (TokenType::Word, "Italiam,".into()),
            (TokenType::Word, "fato".into()),
            (TokenType::Word, "profugus,".into()),
            (TokenType::Word, "Laviniaque".into()),
            (TokenType::Word, "venit".into()),
            (TokenType::Word, "litora,".into()),
            (TokenType::Word, "multum".into()),
            (TokenType::Word, "ille".into()),
            (TokenType::Word, "et".into()),
            (TokenType::Word, "terris".into()),
            (TokenType::Word, "iactatus".into()),
            (TokenType::Word, "et".into()),
            (TokenType::Word, "alto".into()),
            (TokenType::Word, "vi".into()),
            (TokenType::Word, "superum".into()),
            (TokenType::Word, "saevae".into()),
            (TokenType::Word, "memorem".into()),
            (TokenType::Word, "Iunonis".into()),
            (TokenType::Word, "ob".into()),
            (TokenType::Word, "iram".into()),
        ];
        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_comment() {
        let tokens = scan(
            r##"Aeneid % By Virgil
arma virumque cano
%I sing of arms and man
Triae qui"##,
        );
        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Word, "Aeneid".into()),
            (TokenType::Comment, " By Virgil".into()),
            (TokenType::Word, "arma".into()),
            (TokenType::Word, "virumque".into()),
            (TokenType::Word, "cano".into()),
            (TokenType::Comment, "I sing of arms and man".into()),
            (TokenType::Word, "Triae".into()),
            (TokenType::Word, "qui".into()),
        ];

        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_command() {
        let tokens = scan(
            r##"\alpha \beta \gamma
\delta
\epsilon"##,
        );
        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Command, "alpha".into()),
            (TokenType::Command, "beta".into()),
            (TokenType::Command, "gamma".into()),
            (TokenType::Command, "delta".into()),
            (TokenType::Command, "epsilon".into()),
        ];

        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn test_escaped() {
        let tokens = scan(r##"\# \$ \% \^ \& \_ \{ \} \~ \\ \ "##);
        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::EscapedChar, "#".into()),
            (TokenType::EscapedChar, "$".into()),
            (TokenType::EscapedChar, "%".into()),
            (TokenType::EscapedChar, "^".into()),
            (TokenType::EscapedChar, "&".into()),
            (TokenType::EscapedChar, "_".into()),
            (TokenType::EscapedChar, "{".into()),
            (TokenType::EscapedChar, "}".into()),
            (TokenType::EscapedChar, "~".into()),
            (TokenType::DoubleBackslash, "".into()),
            (TokenType::EscapedChar, " ".into()),
        ];
        compare_expected_and_tokens(expected, tokens);
    }

    #[test]
    fn comprehensive_test_1() {
        let tokens = scan(
            r##"\documentclass{article}
\begin{document}
Hello, World! $E=mc^2$ 

\end{document} %This is a comment"##,
        );

        let expected: Vec<(TokenType, String)> = vec![
            (TokenType::Command, "documentclass".into()),
            (TokenType::LeftCurlyBracket, "{".into()),
            (TokenType::Word, "article".into()),
            (TokenType::RightCurlyBracket, "}".into()),
            (TokenType::Command, "begin".into()),
            (TokenType::LeftCurlyBracket, "{".into()),
            (TokenType::Word, "document".into()),
            (TokenType::RightCurlyBracket, "}".into()),
            (TokenType::Word, "Hello,".into()),
            (TokenType::Word, "World!".into()),
            (TokenType::Dollar, "$".into()),
            (TokenType::Word, "E=mc".into()),
            (TokenType::Uptick, "^".into()),
            (TokenType::Word, "2".into()),
            (TokenType::Dollar, "$".into()),
            (TokenType::Newline, "\n".into()),
            (TokenType::Command, "end".into()),
            (TokenType::LeftCurlyBracket, "{".into()),
            (TokenType::Word, "document".into()),
            (TokenType::RightCurlyBracket, "}".into()),
            (TokenType::Comment, "This is a comment".into()),
        ];

        compare_expected_and_tokens(expected, tokens);
    }
}

#[cfg(test)]
mod test_token_token_type {
    use super::*;

    #[test]
    fn test_token_display() {
        let input = r##"\documentclass{article}
\begin{document} 
Hello, World! $E=mc^2$ 
\end{document} %This is a comment"##;
        let tokens = scan(input);

        println!("Input text:\n{}", input);

        println!("Scanned tokens:");
        for i in tokens {
            print!("{}", i);
        }
    }
}
