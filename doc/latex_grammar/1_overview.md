# Latex3e Grammar Overview

## Reserved Characters

There are ten reserved characters in LaTeX3e, they are

|-| Character | Name          | Usage                               |
|-|-----------|---------------|-------------------------------------|
|-| `#`       | Hash          | Parameter in macro definitions      |
|-| `$`       | Dollar        | Math mode delimiter                 |
|-| `%`       | Percent       | Comment character                   |
|-| `&`       | Ampersand     | Alignment tab in tabular environments|
|-| `_`       | Underscore    | Subscript in math mode              |
|-| `^`       | Caret         | Superscript in math mode            |
|-| `{`       | Left brace    | Beginning of a group                |
|-| `}`       | Right brace   | End of a group                      |
|-| `~`       | Tilde         | Non-breaking space                  |
|-| `\`       | Backslash     | Command prefix                      |

In latex, forward slash `/` is treated as a normal character, while backslash `\` is a reserved character.

Backslash is used for starting a latex command and escaping characters (except itself, as `\\` is new line). 
The characters that can be escaped are `# $ % & _ ^ { } ~`, and ` `, the spacce character.

TODO: are there more to be escaped?

Backslash is almost never used alone. When appear by itself, it create space.
See spacing section for more details.

## Latex Spacing and New Lines

Here are the rules for spacing in text mode
1. All tabs are treated as a single space.
1. Any consective spaces are treated as a single space.
1. Any space at the beginning of a line (or control group) is ignored. 
1. Certain special characters are treated as space. These characters are `{}` , `\` by itself (TODO: are there more?)
1. A single new line is treated as a space.
1. Any number of consecutive empty lines is treated as a paragraph break. 
An empty line is a line with no characters or only spaces.
A line with comments is not an empty line.
    1. For example, the following is not an empty line.
    ```
            % this is a comment
    ```

## Comments

In LaTeX, everything after `%` to the end of the line is a comment and is ignored by the compiler. 
There are several caveats, however.

1. If there are none-space character before `%`, characters after `%` are ignored. But the new line `\n` is kept.
1. If the single line consists only of spaces and `%`, the entire line is ignored, including the new line `\n`.
For example
```
a
        % this is a comment
        % this is a comment
b
```
is rendered as `a b`, while
```
a % comment 

b
```
is rendered as 
```
a
b
```

So this is how we parse comments:
1. All comments are treated as spaces.
1. If the current line contains only spaces and comments, the `\n` of this line is ignored 
1. If the current line contains none-space characters before `%`, the `\n` of this line is kept.
