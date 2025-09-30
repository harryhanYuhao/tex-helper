# Tex-helper

Latex-helper is a cli program which provides some convenient utilities for writing latex. 

## Usage

### Initialisation

`tex-helper` helps you initialise a latex project, by creating a `main.tex` with a preamble, a `.gitignore` file, and a sample `references.bib` file.

```sh
tex-helper init <project-name> 
tex-helper init <project-name> --doc-mode=report 
```

The `<project-name>` field is required. 
All files will be created in `<project-name>` directory.

Sample docs mode includes `article`, `report`, `book`, and `letter`. 

You can create a file named `article.tex` in the `~/.config/tex-helper` directory to overwrite default preamble for `article` mode.
In fact, any custom doc-mode can be created, by simple creating a file name `customdoc.tex` or `customdoc` in the `~/.config/tex-helper` directory, and initialising the project with `tex-helper init --doc-mode=customdoc <projectname>`.

If `~/.config/tex-helper/<doc-mode>` is a directory, 
`tex-helper init --doc-mode=customdoc <projectname>` will copy all files in it to `<projectname>` directory recursively. 

More customisation options are yet to come.

### Documentations

Tex-helper's cli facilities are created using the powerful `clap` crate.
Check out the help message for the main command and each subcommand for more details.

```sh
tex-helper -h  # general help
tex-helper init -h  # help for init subcommand
```

## Other tex productivity tools

- Overleaf: online latex editor
