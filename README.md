# talk-timer

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/lukehsiao/talk-timer/rust)](https://github.com/lukehsiao/talk-timer/actions)
[![Crates.io](https://img.shields.io/crates/v/talk-timer)](https://crates.io/crates/talk-timer)
[![License](https://img.shields.io/crates/l/talk-timer)](https://github.com/lukehsiao/talk-timer/blob/master/LICENSE-MIT)

`talk-timer` is a simple command line timer. It simply displays a countdown in your terminal in
blocks of 10 seconds, until the final 10 seconds, which counts down by seconds. Then, it flashes red
when hitting 0. You could magnify your terminal and have this facing a speaker in a conference to
provide them a helpful indicator of how much time they have remaining.

This is essentially a toy, written to explore typestates in Rust.

## Installation

Install using [cargo][cargo]:

```
cargo install talk-timer
```

## Usage

```
talk-timer

USAGE:
  talk-timer <DURATION>

FLAGS:
  -h, --help  Prints this help information

ARGUMENTS:
  DURATION    A duration of time in hours, mins, or secs (e.g., "20m" or "65s")

```

[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
