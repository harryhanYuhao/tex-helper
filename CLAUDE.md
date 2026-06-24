# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build/Run/Test

```sh
cargo build              # debug build
cargo build --release    # release build
cargo run                # run (debug)
cargo test               # run all tests
cargo test -p tex-helper -- scanner   # run tests in a specific module
cargo fmt                # format (rustfmt, max_width=80)
cargo clippy             # lint
```

## Architecture

**tex-helper** is a Rust CLI tool (binary crate) that provides LaTeX productivity utilities. It uses `clap` derive for the CLI interface and `simplelog` for logging.

### Module overview

```
src/
├── main.rs                        # Entry point, calls cli::cli()
├── cli/
│   ├── mod.rs                     # CLI definition (Cli struct, Commands enum), logger init, dispatch
│   ├── init/
│   │   ├── mod.rs                 # `tex-helper init <name> --doc-mode=<mode>` — project scaffolding
│   │   └── default_assets.rs      # Built-in LaTeX preamble templates (article/report/book/letter)
│   ├── format.rs                  # `tex-helper format <target>` — orchestrates the scanner→parser→formatter pipeline
│   └── compile.rs                 # Planned `tex-helper compile` (currently incomplete/stubbed)
├── config.rs                      # Config: CLI flags merged with ~/.config/tex-helper/config.toml
├── utils.rs                       # FileInput struct, file I/O helpers, latex binary detection
├── latex_interpreter/             # LaTeX processing pipeline (frontend-like architecture)
│   ├── token.rs                   # Token and TokenType enums
│   ├── scanner.rs                 # Lexer: &str → Vec<Token> (handles comments, newlines, math, commands)
│   ├── ast.rs                     # AST: Node, NodeType (Passage/Paragraph/Word/Command/Envr/Math/etc.), Walker
│   ├── parser.rs                  # LL(1) recursive descent parser: Vec<Token> → NodePtr (AST root)
│   ├── formatter.rs               # Recursive descent formatter: NodePtr → String (WIP — many node types are stubs)
│   └── error.rs                   # TokenErrList with cargo-style error messages (file:row:col)
└── markdown_interpreter/          # Planned markdown→LaTeX conversion (only scanner implemented so far)
    ├── mod.rs
    └── scanner.rs                 # Markdown tokenizer (separate from LaTeX scanner)
```

### Data flow (format subcommand)

```
.tex file → FileInput → scanner::scan() → Vec<Token>
                      → parser::parse() → NodePtr (AST root)
                      → formatter::format() → String
```

### Key design decisions

- **AST nodes use `Arc<Mutex<Node>>`** (`NodePtr` type alias) — shared ownership, tree traversal via the `Walker` struct.
- **Scanner preserves comments** as tokens (not discarded), since this is a formatter not a compiler.
- **Spacing rules**: single newline = ignored (treated as space), ≥2 newlines = `NewParagraph` token.
- **Config** is loaded from `~/.config/tex-helper/config.toml` (TOML) and merged with CLI flags (debug flag from CLI takes precedence).
- **Custom init templates**: create a file named `<doc-mode>.tex` (or directory `<doc-mode>/`) in `~/.config/tex-helper/` to override the built-in preamble for `tex-helper init --doc-mode=<doc-mode>`.
- **Preamble separation**: the formatter collects `\usepackage` commands into a `used_packages` list in `FormatRes`, enabling reordering/dedup of packages in output.
- **Error handling**: parser accumulates errors into `TokenErrList` rather than failing on the first error. `TokenErrList` implements `Error` and produces cargo-style diagnostics.
- **The formatter is incomplete** — only `format_passage`, `format_paragraph`, `process_usepackage`, and bracket arg formatting are implemented. Math mode, environment, command, and operation formatting return empty `Vec<String>`.
- **`compile.rs`** in the CLI module is dead code (the `Compile` variant is commented out of the `Commands` enum). It depends on finding `latexmk` or `pdflatex` on the system (see `utils::which_latex_binary()`).
- **No Windows path handling yet** — `get_config_dir()` hardcodes `$HOME/.config/tex-helper`.
