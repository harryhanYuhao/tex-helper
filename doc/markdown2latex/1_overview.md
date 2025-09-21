# Markdown to Latex

Tex-helper provides another functionality to convert markdown to LaTeX, as an effort to make the text document more readable.

So far, the implementation is navie

1. Headers are converted to section or subsections
1. Italisized, bold text are kept
1. Math mode is simple inline $$, $$$$, or \[\]

The implemented markdown grammer, based on github flavored markdown, is descriped below. 

## The Grammar of Markdown

### Spacing and Newline 

Markdown spacing and new line is similar to that of latex. That is 

1. Spacing at the beginning of the line is ignored.
1. Consecutive spaces are treated as a single space
1. A single new line is the same as space 
1. One or more empty lines constitutes a linebreak.

(The beginning of a paragraph means that either it is at the beginning of the document, or it is preceeded by one or more empty lines.

### Sections 

Markdown 


