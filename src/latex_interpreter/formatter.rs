use crate::latex_interpreter::ast::*;

#[derive(Debug)]
enum FormatState {
    Preamble,
    Indent(u8),
}

// Recall the definitiion of NodeType below:
//enum NodeType {
//    Passage, // A passage consisists of many paragraphs
//    Paragraph, // A paragraph consists of many Words, operations, etc
//    Word,
//    Operation, // parsing a^b a_c
//    Ampersand, // & are used for alignment in Latex
//    DoubleBackSlash, //  \\
//    LineBreak,       // /n  A single line break is considered as a space
//
//    Command,
//    CurlyBracketArg, // {para}
//    SquareBracketArg,
//
//    InlineMath,
//    DisplayMath,
//
//    Envr, // environment
//
//    Comment,
//}

pub fn format(ast: NodePtr) -> String {
    String::new()
}
