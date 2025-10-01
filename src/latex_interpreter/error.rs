use super::scanner::Token;
use colored::*;

pub fn create_error(token: &Token, input: &str) -> String {
    let mut ret = format!(
        "{} at line {}, column {} with token {}\n",
        "Error".red().bold(),
        token.row + 1,
        token.col + 1,
        token,
    );

    let lines: Vec<String> = input.split('\n').map(|s| s.to_string()).collect();
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

    #[test]
    #[ignore]
    fn test_create_error() {
        let input = r"\documentclass{article}
\begin{document}
Hello, World!
\end{document}
";
        let tokens = scanner::scan(&input);
        println!("{}", create_error(&tokens[0], &input));
    }
}
