use super::token::Token;
use colored::*;
use std::fmt;

use crate::utils::FileInput;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct TokenErrList {
    error_vec: Vec<TokenError>,
    file_input: FileInput,
}

impl TokenErrList {
    pub fn empty(file_input: FileInput) -> Self {
        TokenErrList {
            error_vec: vec![],
            file_input,
        }
    }

    pub fn push(&mut self, token: Token, msg: &str) {
        self.error_vec.push(TokenError::new(&token, msg));
    }

    pub fn is_empty(&self) -> bool {
        self.error_vec.is_empty()
    }
}

impl Error for TokenErrList {}

impl fmt::Display for TokenErrList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn aux(
            token: &Token,
            msg: &str,
            input: &str,
            file_path: &str,
        ) -> String {
            // the row and col stored in Token are 0-indexed
            // when reading the file, however, the row and col are 1-indexed
            // (i.e., the first row is row 1, the first col is col 1)
            // The col number points to the first character of the lexeme
            let row = token.row + 1;
            let col = token.col + 1;
            let red_error = "ERROR".red().bold();

            let mut ret =
                format!("{file_path}:{row}:{col} {red_error}: {msg}\n");

            let in_str = input;
            let lines: Vec<String> =
                in_str.split('\n').map(|s| s.to_string()).collect();

            // if we are at a new line, we need to show the previous lines as well
            // note we do not necessarily knows if the previous line is empty as well,
            // so, for simplicity, we just show the previous 2 lines
            // TODO: improve error handling
            if token.lexeme == "\n" {
                if token.row >= 2 {
                    ret.push_str(&format!(
                        "{}{}\n",
                        ">".red(),
                        lines[token.row - 2]
                    ));
                    ret.push_str(&format!(
                        "{}{}\n",
                        ">".red(),
                        lines[token.row - 1]
                    ));
                } else if token.row == 1 {
                    ret.push_str(&format!("{}\n", lines[token.row - 1]));
                }
            }

            ret.push_str(&format!("{}\n", lines[token.row]));
            // show the indicator under the token pointing to the error
            // if the token has multiple characters, indicater shall cover all
            // characters
            // We are trying to imitate the cargo style error message
            ret.push_str(&format!(
                "{}{}\n",
                " ".repeat(token.col),
                // Some tokens have a shortened lexeme
                // eg, Command(begin) has lexeme "begin", while in the source file it
                // is "\begin"
                // TODO: make sure the 
                "^".repeat(token.lexeme.len()).red().bold()
            ));
            ret
        }

        let input = self.file_input.get_str_content();
        let file_path = self.file_input.get_file_path().to_str().unwrap();
        for error in &self.error_vec {
            writeln!(f, "{}", aux(&error.token, &error.msg, input, file_path))?;
        }
        Ok(())
    }
}

/// Token error holds the Token where error is found and the error messages
/// Tokens contains the row and column numbers, but not the name of the source file, which is
/// stored in FileInput Struct
/// Scanner and parser will have a field of FileInput and
/// Handling the error message is by the corresponding methods of the Scanner and Parser
#[derive(Debug, Clone)]
pub struct TokenError {
    token: Token,
    msg: String,
}

impl TokenError {
    pub fn new(token: &Token, msg: &str) -> Self {
        let token = token.clone();
        let msg = msg.to_string();
        TokenError { token, msg }
    }
}

// pub fn create_error(token_error: &TokenError, input: &FileInput) -> String {
//     let msg = &token_error.msg;
//     let file_path = input.get_file_path().display();
//
//     let token = &token_error.token;
//     let row = token.row + 1;
//     let col = token.col + 1;
//     let red_error = "ERROR".red().bold();
//     let mut ret = format!("{file_path}:{row}:{col} {red_error}: {msg}\n");
//
//     let in_str = input.get_str_content();
//     let lines: Vec<String> =
//         in_str.split('\n').map(|s| s.to_string()).collect();
//
//     ret.push_str(&format!("{}\n", lines[token.row]));
//     ret.push_str(&format!(
//         "{}{}\n",
//         " ".repeat(token.col),
//         "^".repeat(token.lexeme.len()).red().bold()
//     ));
//
//     ret
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::latex_interpreter::parser;
    use crate::latex_interpreter::scanner;
    use crate::utils::FileInput;

    #[test]
    #[ignore]
    fn test_create_error() {
        let input = FileInput::from_str(
            "dummy/path",
            r"\documentclass{article}
\begin{document}
Hello, World!
\end{document}
",
        );
        let tokens = scanner::scan(input.clone()).unwrap();

        let token = &tokens[0];
        let token_error = TokenError::new(token, "Test error");
        let mut parse_error = TokenErrList::empty(input.clone());
        parse_error.push(token.clone(), "Test error");

        println!("{}", parse_error);
        println!("The paniced token is: {}", token);
    }
}
