use super::scanner::Token;
use colored::*;

use crate::utils::FileInput;

pub fn create_error(token: &Token, input: &FileInput, msg: &str) -> String {
    let file_path = input.get_file_path().display();
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
        println!("{}", create_error(&tokens[0], &input, "Test error"));
    }
}
