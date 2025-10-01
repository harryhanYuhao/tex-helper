use super::scanner::Token;
use colored::*;

use crate::utils::FileInput;

/// Token error holds the Token where error is found and the error messages
/// Tokens contains the row and column numbers, but not the name of the source file, which is
/// stored in FileInput Struct
/// Scanner and parser will have a field of FileInput and
/// Handling the error message is by the corresponding methods of the Scanner and Parser
pub struct TokenError {
    pub token: Token,
    pub msg: String,
}

impl TokenError {
    pub fn new(token: &Token, msg: &str) -> Self {
        let token = token.clone();
        let msg = msg.to_string();
        TokenError { token, msg }
    }
}

pub fn create_error(token_error: &TokenError, input: &FileInput) -> String {
    let msg = &token_error.msg;
    let file_path = input.get_file_path().display();

    let token = &token_error.token;
    let row = token.row + 1;
    let col = token.col + 1;
    let red_error = "ERROR".red().bold();
    let mut ret = format!("{file_path}:{row}:{col} {red_error}: {msg}\n");

    let in_str = input.get_content();
    let lines: Vec<String> =
        in_str.split('\n').map(|s| s.to_string()).collect();

    ret.push_str(&format!("{}\n", lines[token.row]));
    ret.push_str(&format!(
        "{}{}\n",
        " ".repeat(token.col),
        "^".repeat(token.lexeme.len()).red().bold()
    ));

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::latex_interpreter::parser;
    use crate::latex_interpreter::scanner;
    use crate::utils::FileInput;

    #[test]
    #[ignore]
    fn test_create_error() {
        let input = FileInput {
            file_path: "dummy/path".into(),
            content: r"\documentclass{article}
\begin{document}
Hello, World!
\end{document}
"
            .into(),
        };
        let tokens = scanner::scan_file(&input);

        let token_error = TokenError::new(&tokens[0], "Test error");
        println!("{}", create_error(&token_error, &input));
    }
}
