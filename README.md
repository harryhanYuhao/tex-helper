# Tex-helper

Many chores are neccessary when writing latex such as creating a long list of `.gitignore`, prepending a long preamble, deleteing all the auxillary files generated when compiling the latex, etc. 
This tool helps you manage these.

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
In fact, any custom doc-mode can be created, by simple creating a file name `customdoc.tex` in the `~/.config/tex-helper` directory, and initialising the project with `tex-helper init --doc-mode=customdoc <projectname>`.

More customisation options are yet to come.

### Documentations

Tex-helper's cli facilities are created using the powerful `clap` crate.
Check out the help message for the main command and each subcommand for more details.

```sh
tex-helper -h  # general help
tex-helper init -h  # help for init subcommand
```

## Todo

- add compatibility for Windows (So far only compatible with Unix)

## Other tex productivity tools

- Overleaf: online latex editor
