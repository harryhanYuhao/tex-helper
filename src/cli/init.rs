use crate::CONFIG;

use lazy_static::lazy_static;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::io::Write;
use colored::Colorize;

lazy_static! {
    static ref article_header: String = String::from("\\documentclass{article}");
    static ref book_header: String = String::from("\\documentclass{book}");
    static ref report_header: String = String::from("\\documentclass{report}");
    static ref letter_header: String = String::from("\\documentclass{letter}");
    static ref default_single_file_preamble: String = String::from(
        r##"

\usepackage[tmargin=2.5cm,rmargin=3cm,lmargin=3cm,bmargin=3cm]{geometry} 
% Top margin, right margin, left margin, bottom margin, footnote skip
\usepackage[utf8]{inputenc}
\usepackage{biblatex}
% \addbibresource{./reference/reference.bib}
% linktocpage shall be added to snippets.
\usepackage{hyperref,theoremref}
\hypersetup{
	colorlinks, 
	linkcolor={red!40!black}, 
	citecolor={blue!50!black},
	urlcolor={blue!80!black},
	linktocpage % Link table of content to the page instead of the title
}

\usepackage{blindtext}
\usepackage{titlesec}
\usepackage{amsthm}
\usepackage{thmtools}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{graphicx}
\usepackage{titlesec}
\usepackage{xcolor}
\usepackage{multicol}
\usepackage{hyperref}
\usepackage{import}


\newtheorem{theorem}{Theorema}[section]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{corollary}{Corollarium}[section]
\newtheorem{proposition}{Propositio}[theorem]
\theoremstyle{definition}
\newtheorem{definition}{Definitio}[section]

\theoremstyle{definition}
\newtheorem{axiom}{Axioma}[section]

\theoremstyle{remark}
\newtheorem{remark}{Observatio}[section]
\newtheorem{hypothesis}{Coniectura}[section]
\newtheorem{example}{Exampli Gratia}[section]

% Proof Environments
\newcommand{\thm}[2]{\begin{theorem}[#1]{}#2\end{theorem}}

%TODO mayby proof environment shall have more margin
\renewenvironment{proof}{\vspace{0.4cm}\noindent\small{\emph{Demonstratio.}}}{\qed\vspace{0.4cm}}
% \renewenvironment{proof}{{\bfseries\emph{Demonstratio.}}}{\qed}
\renewcommand\qedsymbol{Q.E.D.}
% \renewcommand{\chaptername}{Caput}
% \renewcommand{\contentsname}{Index Capitum} % Index Capitum 
\renewcommand{\emph}[1]{\textbf{\textit{#1}}}
\renewcommand{\ker}[1]{\operatorname{Ker}{#1}}

%\DeclareMathOperator{\ker}{Ker}

% New Commands
\newcommand{\bb}[1]{\mathbb{#1}} %TODO add this line to nvim snippets
\newcommand{\orb}[2]{\text{Orb}_{#1}({#2})}
\newcommand{\stab}[2]{\text{Stab}_{#1}({#2})}
\newcommand{\im}[1]{\text{im}{\ #1}}
\newcommand{\se}[2]{\text{send}_{#1}({#2})}

\title{Ars Amatoria}
\author{Publius Ovidius Naso} 
\date{\today}

\begin{document}
Siquis in hoc artem populo non novit amandi, \\
     Hoc legat et lecto carmine doctus amet.
% \maketitle
% \tableofcontents

% \printbibliography
\end{document}"##
    );
}

fn init_template(header: &String, preamble: &String, hint: &str) -> Result<(), Box<dyn Error>> {
    let config = CONFIG.lock().unwrap();
    let file_path = config.get_file_path() + &config.get_main_file_name();

    create_dir_all(&config.get_file_path())?;
    if Path::new(&file_path).exists() {
        return Err(format!("{} already exists", file_path).into());
    }

    // Create or open the file
    let mut file = File::create(&file_path)?;

    let content: String = header.clone() + &(*preamble);
    // Write the string to the file
    file.write_all(content.as_bytes())?;

    println!("Latex {} created at {}", hint, format!("{}", file_path).blue());

    Ok(())
}

pub(super) fn init_article() -> Result<(), Box<dyn Error>> {
    init_template(&article_header, &default_single_file_preamble, "article")
}

pub(super) fn init_book() -> Result<(), Box<dyn Error>> {
    init_template(&book_header, &default_single_file_preamble, "book")
}

pub(super) fn init_report() -> Result<(), Box<dyn Error>> {
    init_template(&report_header, &default_single_file_preamble, "report")
}

pub(super) fn init_letter() -> Result<(), Box<dyn Error>> {
    init_template(&letter_header, &default_single_file_preamble, "letter")
}
