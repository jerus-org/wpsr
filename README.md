# wpsr - Word Puzzle Solver

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-batch]][circleci-url]
[![Rust 1.81+][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/wpsr.svg
[crates-url]: https://crates.io/crates/wpsr
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/wpsr-rs/blob/main/LICENSE
[circleci-batch]: https://dl.circleci.com/status-badge/img/gh/jerus-org/wpsr/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/wpsr/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.85+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/wpsr/badge.svg
[docs-url]:  https://docs.rs/wpsr
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

Word Puzzle Solver (WPSR) is a command line program to solve the word puzzles.

## Installation

Download the source from the git repository and build it with Cargo:

```console
$ git clone https://github.com/jerus-org/wpsr-rs.git
$ cd wpsr-rs
$ cargo build --release

```

Run from the the command line in the project directory for access to the the dictionary words. 

Alternatively, build a bundle with cargo bundle to install and make the dictionary files available from any directory. 

```console
$ cargo bundle --release

```

## Usage

There are four subcommands:
* `alpha` - Parse list of words to exclude duplicates and non-alphabetic characters
* `list` - List available word lists
* `boxed` - Boxed word puzzle tools
* `words` - Generate words from a string of letters

`alpha` is a utility to parse a list of words to exclude duplicates and non-alphabetic characters to create a word list file for use in solving word puzzles.

`list` lists the available word lists default or specified directory.

`words` generates words generates words as solutions for puzzles based on a limited selection of letters. 

`boxed` provides tools to solve and generate puzzles based on the Letters Boxed puzzle. It offers a generator to generate puzzles and solves boxes with between 3 and 8 edges (each of with 3 letters).

```console
$ wpsr --help
Command line program to help solve word puzzles

Usage: wpsr [OPTIONS] <COMMAND>

Commands:
  boxed  Boxed word puzzle tools
  list   List available word lists
  words  Generate words from a string of letters
  alpha  Parse list of words to exclude duplicates and non-alphabetic characters
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version

```

### boxed

The boxed sub-command provides tools to solve and generate puzzles based on the Letters Boxed puzzle.

Letters Boxed is a word puzzle in which the player is presented with a shape with three letters on each edge of the shape. The puzzle is solved by connecting letters from alternating edges to form a chain of words, each next word starting with the last letter of the previous word. The words must be valid English words and must be at least 3 letters long. The words must also be words that can be found in the dictionary.

It offers a generator to generate puzzles and solves shapes with between 3 and 8 edges (each of with 3 letters).

* `prepare` - Prepare word list
* `solutions` - Report multiple solutions for the puzzle
* `solve` - Solve word puzzle
* `generate` - Generate random letter string for puzzle

`prepare` massages the word dictionary to prepare it specifically for solving this type of puzzle by eliminating any word that contains a double letter (such as `letter`) and eliminating words that are shorter than a minimum length (default 3).

`solve` find the shortest solution for the puzzle string comprising of the minimum number of words required to meet the puzzle criteria. 

`solutions` generate multiple solutions for the same puzzle string. 

`generate` will generate a random letter string for a puzzle of any shape from triangle to octagon. Three letters will be selected randomly for each edge of the shape chosen resulting in a string of between 9 and 24 letters.

```console
$ wpsr boxed -h
Boxed word puzzle tools

Usage: wpsr boxed [OPTIONS] <COMMAND>

Commands:
  generate   Generate random letter string for puzzle
  prepare    Prepare word list
  solutions  Report multiple solutions for the puzzle
  solve      Solve word puzzle
  help       Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version

```

