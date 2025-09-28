use crate::utils;
use std::error::Error;
use std::fs;

fn article_header() -> String {
    String::from("\\documentclass{article}")
}

fn report_header() -> String {
    String::from("\\documentclass{report}")
}
fn book_header() -> String {
    String::from("\\documentclass{book}")
}
fn letter_header() -> String {
    String::from("\\documentclass{letter}")
}

fn preamble() -> String {
    String::from(
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

\newtheorem{theorem}{Theorema}[section]
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
\end{document}"##,
    ) // End of String::from
}

pub(super) fn default_preable(doc_mode: &str) -> String {
    match doc_mode {
        "article" => format!("{}{}", article_header(), preamble(),),
        "report" => format!("{}{}", report_header(), preamble(),),
        "book" => format!("{}{}", book_header(), preamble(),),
        "letter" => format!("{}{}", letter_header(), preamble(),),
        _ => String::new(),
    }
}


pub(super) fn reference_bib() -> String {
    String::from(
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
        "##,
    ) // End of String::from
}

pub(super) fn gitignore() -> String {
    String::from(
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
"##,
    ) // End of String::from
}
