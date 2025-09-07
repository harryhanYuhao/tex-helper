// This file contains the `init` command logic 
// this file is not the initialisation of the crate
use crate::CONFIG;

use crate::utils::legal_characters_for_dir_name;
use colored::Colorize;
use lazy_static::lazy_static;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

lazy_static! {
    static ref ARTICLE_HEADER: String = String::from("\\documentclass{article}");
    static ref BOOK_HEADER: String = String::from("\\documentclass{book}");
    static ref REPORT_HEADER: String = String::from("\\documentclass{report}");
    static ref LETTER_HEADER: String = String::from("\\documentclass{letter}");
    static ref GIT_IGNORE_TEMPLATE: String = String::from(
        r##"*.pdf
*.txt
*.aux
*.gz
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
*.bbl
"##
    );
    static ref REFERENCE_BIB_TEMPLATE: String = String::from(
        r##"% @inproceedings{lesk:1977,
%   title={Computer Typesetting of Technical Journals on {UNIX}},
%   author={Michael Lesk and Brian Kernighan},
%   booktitle={Proceedings of American Federation of
%              Information Processing Societies: 1977
%              National Computer Conference},
%   pages={879--888},
%   year={1977},
%   address={Dallas, Texas}
% }
% @article{knuth:1984,
%   title={Literate Programming}.
%   author={Donald E. Knuth},
%   journal={The Computer Journal},
%   volume={27},
%   number={2},
%   pages={97--111},
%   year={1984},
%   publisher={Oxford University Press}
% }

% All possible entries are 
% article, book, booklet, conference, inbook, incollection, inproceedings, manual, mastersthesis, misc, phdthesis, proceedings, techreport, unpublished
% Possible fields are 
% address, annote, author, booktitle, chapter, crossref, edition, editor, howpublished, institution, journal, key, month, note, number, organization, pages, publisher, school, series, title, type, volume, year, url, doi, isbn, issn


@misc {Ovid,
	author = {Ovid},
	title = {Metamorphoses  Liber Primus},
	url = {https://www.thelatinlibrary.com/ovid/ovid.artis1.shtml}
}
        "##
    );
    static ref SINGLE_FILE_PREAMBLE_TEMPLATE: String = String::from(
        r##"

\usepackage[tmargin=2.5cm,rmargin=3cm,lmargin=3cm,bmargin=3cm]{geometry} 
% Top margin, right margin, left margin, bottom margin, footnote skip
\usepackage[utf8]{inputenc}
\usepackage{biblatex}
\addbibresource{./references.bib}
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

\newtheorem{theorem}{Theorema}[chapter]
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{corollary}[theorem]{Corollarium}
\newtheorem{proposition}[theorem]{Propositio}
\theoremstyle{definition}
\newtheorem{definition}[theorem]{Definitio}

\theoremstyle{definition}
\newtheorem{axiom}[theorem]{Axioma}
\newtheorem{observation}[theorem]{Observation}

\theoremstyle{remark}
\newtheorem{remark}[theorem]{Observatio}
\newtheorem{hypothesis}[theorem]{Coniectura}
\newtheorem{example}[theorem]{Exampli Gratia}

% Proof Environments
\newcommand{\thm}[2]{\begin{theorem}[#1]{}#2\end{theorem}}

%TODO mayby proof environment shall have more margin
%\renewenvironment{proof}{\vspace{0.4cm}\noindent\small{\emph{Demonstratio.}}}{\qed\vspace{0.4cm}}
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
     Tiphys et Automedon dicar Amoris ego.\cite{Ovid}\\

Should anyone here not know the art of love,\\
read this, and learn by reading how to love.

By art the boat’s set gliding, with oar and sail,\\
by art the chariot’s swift: love’s ruled by art.

Automedon was skilled with Achilles’s chariot reins,\\
Tiphys in Thessaly was steersman of the Argo,

Venus appointed me as guide to gentle Love:\\
I’ll be known as Love’s Tiphys, and Automedon.

\printbibliography
\end{document}"##
    );
}

// auxiliary function: shall only be called wihthin the crate with simple logics
// TODO: Add support for windows
fn file_path_from_dir_and_filename(directory: &str, filename: &str) -> String {
    if directory.len() == 0 {
        panic!("file_path_string() called with directory string empty!");
    }
    if filename.len() == 0 {
        panic!("file_path_string() called with filename string empty!");
    }
    let last_char = directory.chars().last().unwrap();
    if last_char == '/' {
        return format!("{}{}", directory, filename);
    }
    format!("{}/{}", directory, filename)
}

// This function only creates files if it does not exists,
// return error if the file does exists
fn create_new_files_if_not_exist_error_otherwise(
    filepath: &str,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    if Path::new(filepath).exists() {
        return Err(format!(
            "{} already exists when calling create_new_files_if_not_exist()!",
            filepath
        )
        .into());
    }

    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn init_template(
    package_name: &str,
    header: &str,
    preamble: &str,
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

    // NOTE: package_name is also the directory name
    if Path::new(&package_name).exists() {
        return Err(format!("{} already exists", package_name).into());
    }

    let main_file_name = config.get_main_file_name();
    let file_path = format!("{}/{}", package_name, main_file_name);

    create_dir_all(package_name)?;

    // create gitignore
    create_new_files_if_not_exist_error_otherwise(
        &file_path_from_dir_and_filename(package_name, ".gitignore"),
        &(*GIT_IGNORE_TEMPLATE),
    )?;

    // create references.bib
    create_new_files_if_not_exist_error_otherwise(
        &file_path_from_dir_and_filename(package_name, "references.bib"),
        &(*REFERENCE_BIB_TEMPLATE),
    )?;

    // Create or open the file
    let mut file = File::create(&file_path)?;

    let content: String = header.to_owned() + &(*preamble);
    // Write the string to the file
    file.write_all(content.as_bytes())?;

    println!(
        "Latex {} project created: {}",
        hint,
        format!("{}/", package_name).blue()
    );

    Ok(())
}

pub(super) fn init_article(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(
        name,
        &ARTICLE_HEADER,
        &SINGLE_FILE_PREAMBLE_TEMPLATE,
        "article",
    )
}

pub(super) fn init_book(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(name, &BOOK_HEADER, &SINGLE_FILE_PREAMBLE_TEMPLATE, "book")
}

pub(super) fn init_report(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(
        name,
        &REPORT_HEADER,
        &SINGLE_FILE_PREAMBLE_TEMPLATE,
        "report",
    )
}

pub(super) fn init_letter(name: &str) -> Result<(), Box<dyn Error>> {
    init_template(
        name,
        &LETTER_HEADER,
        &SINGLE_FILE_PREAMBLE_TEMPLATE,
        "letter",
    )
}
