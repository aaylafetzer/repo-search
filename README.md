[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/GPL-3.0)
[![Actions Status](https://github.com/aaylafetzer/RustCloneGitProfile/workflows/Build/badge.svg)](https://github.com/aaylafetzer/repo-search/actions)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/aaylafetzer/repo-search)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/aaylafetzer/repo-search)
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