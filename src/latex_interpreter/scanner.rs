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
    Space,
    // The job of making two newlines into a paragraph is left to the parser
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
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].lexeme, "a");

        assert_eq!(tokens[1].token_type, TokenType::Space);
        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].lexeme, "bc");

        assert_eq!(tokens[3].token_type, TokenType::Space);
        assert_eq!(tokens[4].token_type, TokenType::Word);
        assert_eq!(tokens[4].lexeme, "d");

        assert_eq!(tokens[5].token_type, TokenType::Space);
    }

    #[test]
    fn test_fnscan_newline() {
        let tokens = scan("a\nb");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].lexeme, "a");

        assert_eq!(tokens[1].token_type, TokenType::Newline);

        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].lexeme, "b");

        let tokens = scan("%\nb");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Comment);

        assert_eq!(tokens[1].token_type, TokenType::Newline);

        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].lexeme, "b");

        let tokens = scan(
            r##"a %
%
aaa"##,
        );
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].lexeme, "a");
        assert_eq!(tokens[1].token_type, TokenType::Space);

        assert_eq!(tokens[2].token_type, TokenType::Comment);
        assert_eq!(tokens[3].token_type, TokenType::Newline);
        assert_eq!(tokens[4].token_type, TokenType::Comment);
        assert_eq!(tokens[5].token_type, TokenType::Newline);
        assert_eq!(tokens[6].token_type, TokenType::Word);
        assert_eq!(tokens[6].lexeme, "aaa");
    }

    #[test]
    fn test_slash_bracket() {
        let tokens = scan(r"\[ \]");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::SlashOpenBracket);
        assert_eq!(tokens[1].token_type, TokenType::Space);
        assert_eq!(tokens[2].token_type, TokenType::SlashCloseBracket);
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
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].lexeme, "arma");
        assert_eq!(tokens[1].token_type, TokenType::Space);

        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].lexeme, "virumque");
        assert_eq!(tokens[3].token_type, TokenType::Space);

        assert_eq!(tokens[4].token_type, TokenType::Word);
        assert_eq!(tokens[4].lexeme, "cano");
        assert_eq!(tokens[5].token_type, TokenType::Space);

        assert_eq!(tokens[6].token_type, TokenType::Word);
        assert_eq!(tokens[6].lexeme, ",");
        assert_eq!(tokens[7].token_type, TokenType::Space);
    }
    #[test]
    fn test_long_text() {
        let tokens = scan(
            r##"arma virumque cano, Troiae qui primus ab oris 
Italiam, fato profugus, Laviniaque venit 
litora, multum ille et terris iactatus et alto 
vi superum saevae memorem Iunonis ob iram"##,
        );

        assert_eq!(tokens.len(), 58);
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].lexeme, "arma");
        assert_eq!(tokens[1].token_type, TokenType::Space);

        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].lexeme, "virumque");
        assert_eq!(tokens[3].token_type, TokenType::Space);

        assert_eq!(tokens[4].token_type, TokenType::Word);
        assert_eq!(tokens[4].lexeme, "cano,");
        assert_eq!(tokens[5].token_type, TokenType::Space);

        assert_eq!(tokens[6].token_type, TokenType::Word);
        assert_eq!(tokens[6].lexeme, "Troiae");
        assert_eq!(tokens[7].token_type, TokenType::Space);

        assert_eq!(tokens[8].token_type, TokenType::Word);
        assert_eq!(tokens[8].lexeme, "qui");
        assert_eq!(tokens[9].token_type, TokenType::Space);

        assert_eq!(tokens[10].token_type, TokenType::Word);
        assert_eq!(tokens[10].lexeme, "primus");
        assert_eq!(tokens[11].token_type, TokenType::Space);

        assert_eq!(tokens[12].token_type, TokenType::Word);
        assert_eq!(tokens[12].lexeme, "ab");
        assert_eq!(tokens[13].token_type, TokenType::Space);

        assert_eq!(tokens[14].token_type, TokenType::Word);
        assert_eq!(tokens[14].lexeme, "oris");

        assert_eq!(tokens[15].token_type, TokenType::Space);
        assert_eq!(tokens[16].token_type, TokenType::Newline);

        // Continue with the second line
        assert_eq!(tokens[17].token_type, TokenType::Word);
        assert_eq!(tokens[17].lexeme, "Italiam,");
        assert_eq!(tokens[18].token_type, TokenType::Space);

        assert_eq!(tokens[19].token_type, TokenType::Word);
        assert_eq!(tokens[19].lexeme, "fato");
        assert_eq!(tokens[20].token_type, TokenType::Space);

        assert_eq!(tokens[21].token_type, TokenType::Word);
        assert_eq!(tokens[21].lexeme, "profugus,");
        assert_eq!(tokens[22].token_type, TokenType::Space);

        assert_eq!(tokens[23].token_type, TokenType::Word);
        assert_eq!(tokens[23].lexeme, "Laviniaque");
        assert_eq!(tokens[24].token_type, TokenType::Space);

        assert_eq!(tokens[25].token_type, TokenType::Word);
        assert_eq!(tokens[25].lexeme, "venit");
        assert_eq!(tokens[26].token_type, TokenType::Space);

        assert_eq!(tokens[27].token_type, TokenType::Newline);

        // Third line
        assert_eq!(tokens[28].token_type, TokenType::Word);
        assert_eq!(tokens[28].lexeme, "litora,");
        assert_eq!(tokens[29].token_type, TokenType::Space);

        assert_eq!(tokens[30].token_type, TokenType::Word);
        assert_eq!(tokens[30].lexeme, "multum");
        assert_eq!(tokens[31].token_type, TokenType::Space);

        assert_eq!(tokens[32].token_type, TokenType::Word);
        assert_eq!(tokens[32].lexeme, "ille");
        assert_eq!(tokens[33].token_type, TokenType::Space);

        assert_eq!(tokens[34].token_type, TokenType::Word);
        assert_eq!(tokens[34].lexeme, "et");
        assert_eq!(tokens[35].token_type, TokenType::Space);

        assert_eq!(tokens[36].token_type, TokenType::Word);
        assert_eq!(tokens[36].lexeme, "terris");
        assert_eq!(tokens[37].token_type, TokenType::Space);

        assert_eq!(tokens[38].token_type, TokenType::Word);
        assert_eq!(tokens[38].lexeme, "iactatus");
        assert_eq!(tokens[39].token_type, TokenType::Space);

        assert_eq!(tokens[40].token_type, TokenType::Word);
        assert_eq!(tokens[40].lexeme, "et");
        assert_eq!(tokens[41].token_type, TokenType::Space);

        assert_eq!(tokens[42].token_type, TokenType::Word);
        assert_eq!(tokens[42].lexeme, "alto");
        assert_eq!(tokens[43].token_type, TokenType::Space);

        assert_eq!(tokens[44].token_type, TokenType::Newline);

        // Fourth line
        assert_eq!(tokens[45].token_type, TokenType::Word);
        assert_eq!(tokens[45].lexeme, "vi");
        assert_eq!(tokens[46].token_type, TokenType::Space);

        assert_eq!(tokens[47].token_type, TokenType::Word);
        assert_eq!(tokens[47].lexeme, "superum");
        assert_eq!(tokens[48].token_type, TokenType::Space);

        assert_eq!(tokens[49].token_type, TokenType::Word);
        assert_eq!(tokens[49].lexeme, "saevae");
        assert_eq!(tokens[50].token_type, TokenType::Space);

        assert_eq!(tokens[51].token_type, TokenType::Word);
        assert_eq!(tokens[51].lexeme, "memorem");
        assert_eq!(tokens[52].token_type, TokenType::Space);

        assert_eq!(tokens[53].token_type, TokenType::Word);
        assert_eq!(tokens[53].lexeme, "Iunonis");
        assert_eq!(tokens[54].token_type, TokenType::Space);

        assert_eq!(tokens[55].token_type, TokenType::Word);
        assert_eq!(tokens[55].lexeme, "ob");
        assert_eq!(tokens[56].token_type, TokenType::Space);

        assert_eq!(tokens[57].token_type, TokenType::Word);
        assert_eq!(tokens[57].lexeme, "iram");
    }

    #[test]
    fn test_comment() {
        let tokens = scan(
            r##"Aeneid % By Virgil
arma virumque cano
%I sing of arms and man
Triae qui"##,
        );
        assert_eq!(tokens.len(), 15);
        assert_eq!(tokens[0].token_type, TokenType::Word);
        assert_eq!(tokens[0].lexeme, "Aeneid");
        assert_eq!(tokens[1].token_type, TokenType::Space);
        assert_eq!(tokens[2].token_type, TokenType::Comment);
        assert_eq!(tokens[2].lexeme, " By Virgil");

        assert_eq!(tokens[3].token_type, TokenType::Newline);
        assert_eq!(tokens[4].token_type, TokenType::Word);
        assert_eq!(tokens[4].lexeme, "arma");
        assert_eq!(tokens[5].token_type, TokenType::Space);
        assert_eq!(tokens[6].token_type, TokenType::Word);
        assert_eq!(tokens[6].lexeme, "virumque");
        assert_eq!(tokens[7].token_type, TokenType::Space);
        assert_eq!(tokens[8].token_type, TokenType::Word);
        assert_eq!(tokens[8].lexeme, "cano");

        assert_eq!(tokens[9].token_type, TokenType::Newline);
        assert_eq!(tokens[10].token_type, TokenType::Comment);
        assert_eq!(tokens[10].lexeme, "I sing of arms and man");
        assert_eq!(tokens[11].token_type, TokenType::Newline);

        assert_eq!(tokens[12].token_type, TokenType::Word);
        assert_eq!(tokens[12].lexeme, "Triae");
        assert_eq!(tokens[13].token_type, TokenType::Space);
        assert_eq!(tokens[14].token_type, TokenType::Word);
        assert_eq!(tokens[14].lexeme, "qui");
    }

    #[test]
    fn test_command() {
        let tokens = scan(
            r##"\alpha \beta \gamma
\delta
\epsilon"##,
        );
        assert_eq!(tokens.len(), 9);

        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[0].lexeme, r"alpha");
        assert_eq!(tokens[1].token_type, TokenType::Space);

        assert_eq!(tokens[2].token_type, TokenType::Command);
        assert_eq!(tokens[2].lexeme, r"beta");
        assert_eq!(tokens[3].token_type, TokenType::Space);

        assert_eq!(tokens[4].token_type, TokenType::Command);
        assert_eq!(tokens[4].lexeme, r"gamma");

        assert_eq!(tokens[5].token_type, TokenType::Newline);
        assert_eq!(tokens[6].token_type, TokenType::Command);
        assert_eq!(tokens[6].lexeme, r"delta");

        assert_eq!(tokens[7].token_type, TokenType::Newline);
        assert_eq!(tokens[8].token_type, TokenType::Command);
        assert_eq!(tokens[8].lexeme, r"epsilon");
    }

    #[test]
    fn test_escaped() {
        let tokens = scan(r##"\# \$ \% \^ \& \_ \{ \} \~ \\ \ "##);
        assert_eq!(tokens.len(), 21);

        assert_eq!(tokens[0].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[0].lexeme, r"#");
        assert_eq!(tokens[1].token_type, TokenType::Space);

        assert_eq!(tokens[2].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[2].lexeme, r"$");
        assert_eq!(tokens[3].token_type, TokenType::Space);

        assert_eq!(tokens[4].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[4].lexeme, r"%");
        assert_eq!(tokens[5].token_type, TokenType::Space);

        assert_eq!(tokens[6].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[6].lexeme, r"^");
        assert_eq!(tokens[7].token_type, TokenType::Space);

        assert_eq!(tokens[8].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[8].lexeme, r"&");
        assert_eq!(tokens[9].token_type, TokenType::Space);

        assert_eq!(tokens[10].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[10].lexeme, r"_");
        assert_eq!(tokens[11].token_type, TokenType::Space);

        assert_eq!(tokens[12].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[12].lexeme, r"{");
        assert_eq!(tokens[13].token_type, TokenType::Space);

        assert_eq!(tokens[14].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[14].lexeme, r"}");
        assert_eq!(tokens[15].token_type, TokenType::Space);

        assert_eq!(tokens[16].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[16].lexeme, r"~");
        assert_eq!(tokens[17].token_type, TokenType::Space);

        assert_eq!(tokens[18].token_type, TokenType::DoubleBackslash);
        assert_eq!(tokens[19].token_type, TokenType::Space);

        assert_eq!(tokens[20].token_type, TokenType::EscapedChar);
        assert_eq!(tokens[20].lexeme, r" ");
    }

    #[test]
    fn comprehensive_test_1() {
        let tokens = scan(
            r##"\documentclass{article}
\begin{document}
Hello, World! $E=mc^2$ 
\end{document} %This is a comment"##,
        );
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 27);

        // 1st line
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[0].lexeme, r"documentclass");
        assert_eq!(tokens[1].token_type, TokenType::LeftCurlyBracket);
        assert_eq!(tokens[2].token_type, TokenType::Word);
        assert_eq!(tokens[2].lexeme, "article");
        assert_eq!(tokens[3].token_type, TokenType::RightCurlyBracket);
        assert_eq!(tokens[4].token_type, TokenType::Newline);

        // 2nd line
        assert_eq!(tokens[5].token_type, TokenType::Command);
        assert_eq!(tokens[5].lexeme, r"begin");
        assert_eq!(tokens[6].token_type, TokenType::LeftCurlyBracket);
        assert_eq!(tokens[7].token_type, TokenType::Word);
        assert_eq!(tokens[7].lexeme, "document");
        assert_eq!(tokens[8].token_type, TokenType::RightCurlyBracket);
        assert_eq!(tokens[9].token_type, TokenType::Newline);

        // 3rd line
        assert_eq!(tokens[10].token_type, TokenType::Word);
        assert_eq!(tokens[10].lexeme, "Hello,");
        assert_eq!(tokens[11].token_type, TokenType::Space);
        assert_eq!(tokens[12].token_type, TokenType::Word);
        assert_eq!(tokens[12].lexeme, "World!");
        assert_eq!(tokens[13].token_type, TokenType::Space);

        // 4th line
        assert_eq!(tokens[14].token_type, TokenType::Dollar);
        assert_eq!(tokens[15].token_type, TokenType::Word);
        assert_eq!(tokens[15].lexeme, "E=mc");
        assert_eq!(tokens[16].token_type, TokenType::Uptick);
        assert_eq!(tokens[17].token_type, TokenType::Word);
        assert_eq!(tokens[17].lexeme, "2");
        assert_eq!(tokens[18].token_type, TokenType::Dollar);
        assert_eq!(tokens[19].token_type, TokenType::Space);
        assert_eq!(tokens[20].token_type, TokenType::Newline);

        // 5th line
        assert_eq!(tokens[21].token_type, TokenType::Command);
        assert_eq!(tokens[21].lexeme, r"end");
        assert_eq!(tokens[22].token_type, TokenType::LeftCurlyBracket);
        assert_eq!(tokens[23].token_type, TokenType::Word);
        assert_eq!(tokens[23].lexeme, "document");
        assert_eq!(tokens[24].token_type, TokenType::RightCurlyBracket);
        // Trailing comment
        assert_eq!(tokens[25].token_type, TokenType::Space);

        assert_eq!(tokens[26].token_type, TokenType::Comment);
        assert_eq!(tokens[26].lexeme, "This is a comment");
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
