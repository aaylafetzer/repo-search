# Repo Search
A program to search the commit history of a git repository and search for sensitive information via regular expressions.

## Why not use trufflehog?
Because Trufflehog doesn't search for what I want and I'd rather make something that can search for anything I could ever want than fork Trufflehog. 

## Usage
This menu can also be accessed via ``reposearch -h``
```
reposearch 0.0.1
Aayla Fetzer <aayla.fetzer@gmail.com>
Searches git repositories for configurable regular expressions

USAGE:
    reposearch <EXPRESSIONS> <REPO>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <EXPRESSIONS>    Path to the regular expressions to search for
    <REPO>           Path to the git repository
```

Pass your regular expressions to the program with the ``EXPRESSIONS`` argument, formatted as a simple text file with one expression per line.