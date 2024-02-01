#![deny(unsafe_code)]
#![deny(warnings)]

//! # Talk Timer
//! This is a simple command line tool that displays a timer. Built as a toy to
//! explore typestates in Rust, and used as a prompt for speakers during talks.

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::{process, thread};

use anyhow::{anyhow, bail, Error, Result};
use yansi::{Color, Paint};

struct Init;
struct Countdown;
struct LastStretch;
struct Done;

struct Timer<S> {
    duration: Duration,
    start: Instant,
    _state: S,
}

const HELP: &str = "\
talk-timer

USAGE:
  talk-timer <DURATION>

FLAGS:
  -h, --help  Prints this help information

ARGUMENTS:
  DURATION    A duration of time in hours, mins, or secs (e.g., \"20m\" or \"65s\")
";

impl Timer<Init> {
    /// Transition to countdown.
    ///
    /// This method consumes the sender in its current state,
    /// returns it in a new state.
    fn start(self) -> Timer<Countdown> {
        let sec_remaining = (self.duration - self.start.elapsed()).as_secs();

        // To clear space in the terminal so zooming in has no other content
        println!("\n\n");

        print!("{:02}:{:02} left", sec_remaining / 60, sec_remaining % 60,);
        io::stdout().flush().unwrap();
        Timer {
            duration: self.duration,
            start: self.start,
            _state: Countdown,
        }
    }
}

impl Timer<Countdown> {
    /// Run down the timer until time is out.
    fn run(&self) -> Timer<LastStretch> {
        loop {
            thread::sleep(Duration::from_millis(100));
            let remaining = self.duration - self.start.elapsed();
            match remaining.as_secs() {
                s if s < 11 => {
                    break;
                }
                sec_remaining => {
                    if (sec_remaining % 10) == 0 {
                        print!("\r{:02}:{:02} left", sec_remaining / 60, sec_remaining % 60,);
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }

        Timer {
            duration: self.duration,
            start: self.start,
            _state: LastStretch,
        }
    }
}

impl Timer<LastStretch> {
    /// Run down the timer until time is out.
    fn run(&self) -> Timer<Done> {
        loop {
            thread::sleep(Duration::from_millis(100));
            let remaining = self.duration - self.start.elapsed();
            match remaining.as_secs() {
                0 => {
                    print!("\r{}", Paint::red("00:00 left"));
                    io::stdout().flush().unwrap();
                    break;
                }
                sec_remaining => {
                    print!("\r{:02}:{:02} left", sec_remaining / 60, sec_remaining % 60,);
                    io::stdout().flush().unwrap();
                }
            }
        }

        Timer {
            duration: self.duration,
            start: self.start,
            _state: Done,
        }
    }
}

impl Timer<Done> {
    /// Flash warning until program is killed
    fn wait(&self) {
        let mut now = Instant::now();
        let mut toggle = true;
        loop {
            thread::sleep(Duration::from_millis(100));
            let elapsed_ms = now.elapsed().as_millis();
            if elapsed_ms > 500 {
                if toggle {
                    print!("\r{}", Paint::white("00:00 left").bg(Color::Red));
                    io::stdout().flush().unwrap();
                    toggle = !toggle;
                    now = Instant::now()
                } else {
                    print!("\r{}", Paint::red("00:00 left"));
                    io::stdout().flush().unwrap();
                    toggle = !toggle;
                    now = Instant::now()
                }
            }
        }
    }
}

trait TryFrom<T>: Sized {
    type Error;

    fn try_from_str(value: &str) -> Result<Self, Self::Error>;
}

impl TryFrom<&str> for Duration {
    type Error = Error;

    fn try_from_str(value: &str) -> Result<Self, Self::Error> {
        let chars: Vec<char> = value.chars().collect();
        let len = chars.len();
        match value[..len - 1].parse::<u64>() {
            Ok(duration) => match chars[len - 1] {
                'h' | 'H' => Ok(Duration::from_secs(duration * 60 * 60)),
                'm' | 'M' => Ok(Duration::from_secs(duration * 60)),
                's' | 'S' => Ok(Duration::from_secs(duration)),
                _ => Err(anyhow!(
                    "Invalid duration unit. Did you use 's', 'm', or 'h'?"
                )),
            },
            Err(_) => Err(anyhow!("Invalid duration value.")),
        }
    }
}

fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        eprintln!("{}", HELP);
        process::exit(0);
    }

    let dur_str: String = match args.free_from_str() {
        Ok(dur) => dur,
        Err(e) => {
            bail!("[ERROR] Invalid duration: {}", e);
        }
    };

    // Arguments can be parsed in any order.
    let duration = Duration::try_from_str(&dur_str)? + Duration::from_secs(1);
    let start = Instant::now();

    let timer = Timer {
        duration,
        start,
        _state: Init,
    };

    let countdown = timer.start();
    let laststretch = countdown.run();
    let done = laststretch.run();
    done.wait();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_seconds() {
        let duration = Duration::try_from_str("30s").unwrap();
        assert_eq!(duration, Duration::from_secs(30));
    }

    #[test]
    fn test_parse_minutes() {
        let duration = Duration::try_from_str("20M").unwrap();
        assert_eq!(duration, Duration::from_secs(1200));
    }

    #[test]
    fn test_parse_hours() {
        let duration = Duration::try_from_str("2h").unwrap();
        assert_eq!(duration, Duration::from_secs(60 * 60 * 2));
    }

    #[test]
    fn test_reject_units() {
        let duration = Duration::try_from_str("2k");
        assert!(duration.is_err());
    }

    #[test]
    fn test_reject_value() {
        let duration = Duration::try_from_str("somethingh");
        assert!(duration.is_err());
    }
}
