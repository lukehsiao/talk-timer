<h1 align="center">
    ⏲️<br>
    talk-timer
</h1>
<div align="center">
    <strong>A command line countdown timer for talks.</strong>
</div>
<br>
<div align="center">
  <a href="https://github.com/lukehsiao/talk-timer/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/lukehsiao/talk-timer/rust.yml" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/talk-timer">
    <img src="https://img.shields.io/crates/v/talk-timer" alt="Version">
  </a>
  <a href="https://github.com/lukehsiao/talk-timer/blob/main/LICENSE">
    <img src="https://img.shields.io/crates/l/talk-timer" alt="License">
  </a>
</div>
<br>

`talk-timer` is a simple command line countdown timer.
It simply displays a countdown in your terminal in blocks of 10 seconds, until the final 10 seconds, which counts down by seconds.
Then, it flashes red when hitting 0.

I find this useful to set a timer, then magnify (zoom in) on my terminal and have my laptop facing a speaker in a conference to provide them a subtle indicator of how much time they have remaining.

This is essentially a toy, written to explore [typestates](https://yoric.github.io/post/rust-typestate/) in Rust.

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
