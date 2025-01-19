use crate::CONFIG;

use crate::utils::legal_characters_for_dir_name;
use colored::Colorize;
use lazy_static::lazy_static;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

lazy_static! {
    static ref article_header: String = String::from("\\documentclass{article}");
    static ref book_header: String = String::from("\\documentclass{book}");
    static ref report_header: String = String::from("\\documentclass{report}");
    static ref letter_header: String = String::from("\\documentclass{letter}");
    static ref git_ignore_template: String = String::from(
        r##"*.pdf
*.txt
*.ipynb
*.aux
*..gz
*.latexmk
*.fls
*.fdb_latexmk
*.synctex.gz
*.blg
*.log
*.out
*.toc
*.nav
*.snm
*.vrb
*.synctex.gz
*.bcf
*.xml
*.dvi
"##
    );
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

\title{Ars Amatoriae}
\author{Publius Ovidius Naso} 
\date{\today}

\begin{document}
\maketitle
% \tableofcontents

Siquis in hoc artem populo non novit amandi,\\
     Hoc legat et lecto carmine doctus amet.

Arte citae veloque rates remoque moventur,\\
     Arte leves currus: arte regendus amor.

Curribus Automedon lentisque erat aptus habenis\\
     Tiphys in Haemonia puppe magister erat:

Me Venus artificem tenero praefecit Amori;\\
     Tiphys et Automedon dicar Amoris ego.\\

% \printbibliography
\end{document}"##
    );
}

fn create_git_ignore(directory: &str) -> Result<(), Box<dyn Error>> {
    let git_ignore_path = directory.to_string() + "/" + ".gitignore";
    let mut file = File::create(&git_ignore_path)?;
    file.write_all((*git_ignore_template).as_bytes())?;
    Ok(())
}

fn init_template(
    package_name: &str,
    header: &String,
    preamble: &String,
    hint: &str,
) -> Result<(), Box<dyn Error>> {
    let config = CONFIG.lock().unwrap();
    // Error handling
    if legal_characters_for_dir_name(package_name).len() != 0 {
        return Err(format!(
            "{} is an illegal name for directory as it contains {:?}",
            package_name,
            legal_characters_for_dir_name(package_name)
        )
        .into());
    }

    if Path::new(&package_name).exists() {
        return Err(format!("{} already exists", package_name).into());
    }

    let main_file_name = config.get_main_file_name();
    let file_path = format!("{}/{}", package_name, main_file_name);

    create_dir_all(package_name)?;
    create_git_ignore(package_name)?;

    // Create or open the file
    let mut file = File::create(&file_path)?;

    let content: String = header.clone() + &(*preamble);
    // Write the string to the file
    file.write_all(content.as_bytes())?;

    println!(
        "Latex {} project created: {}", hint,
        format!("{}/", package_name).blue()
    );

    Ok(())
}

pub(super) fn init_article(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(
        name,
        &article_header,
        &default_single_file_preamble,
        "article",
    )
}

pub(super) fn init_book(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(name, &book_header, &default_single_file_preamble, "book")
}

pub(super) fn init_report(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(
        name,
        &report_header,
        &default_single_file_preamble,
        "report",
    )
}

pub(super) fn init_letter(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(
        name,
        &letter_header,
        &default_single_file_preamble,
        "letter",
    )
}
