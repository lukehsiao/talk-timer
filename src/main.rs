#![forbid(unsafe_code)]
#![forbid(warnings)]

use chrono;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use yansi::{Color, Paint};

trait TryFrom<T>: Sized {
    type Error;

    fn try_from(value: &str) -> Result<Self, Self::Error>;
}

impl TryFrom<&str> for Duration {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars: Vec<char> = value.chars().collect();
        let len = chars.len();
        match value[..len - 1].parse::<u64>() {
            Ok(duration) => match chars[len - 1] {
                'h' | 'H' => Ok(Duration::from_secs(duration * 60 * 60)),
                'm' | 'M' => Ok(Duration::from_secs(duration * 60)),
                's' | 'S' => Ok(Duration::from_secs(duration)),
                _ => Err("Invalid duration unit. Did you use 's', 'm', or 'h'?"),
            },
            Err(_) => Err("Invalid duration value."),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = pico_args::Arguments::from_env();

    let free = args.free()?;

    if free.len() != 1 {
        eprintln!(
            "
USAGE:
    timer <duration>

ARGUMENTS:
    duration    A duration of time in hours, minutes, or seconds (e.g., \"20m\" or \"65s\")
"
        )
    }

    // Arguments can be parsed in any order.
    let duration = Duration::try_from(free[0].as_str())? + Duration::from_secs(1);

    // Loop as time elapses and update timer
    let start = Instant::now();
    let mut remaining = duration - start.elapsed();
    let mut formatter = chrono::Duration::from_std(remaining)?;
    print!(
        "{:02}:{:02} left\r",
        formatter.num_minutes(),
        formatter.num_seconds() % 60,
    );
    io::stdout().flush().unwrap();
    loop {
        thread::sleep(Duration::from_millis(100));
        remaining = duration - start.elapsed();
        if remaining.as_secs() == 0 {
            print!("{}\n", Paint::white("00:00 left").bg(Color::Red));
            break;
        }
        if (remaining.as_secs() % 10) == 0 {
            formatter = chrono::Duration::from_std(remaining)?;
            print!(
                "{:02}:{:02} left\r",
                formatter.num_minutes(),
                formatter.num_seconds() % 60
            );
            io::stdout().flush().unwrap();
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_seconds() {
        let duration = Duration::try_from("30s").unwrap();
        assert_eq!(duration, Duration::from_secs(30));
    }

    #[test]
    fn test_parse_minutes() {
        let duration = Duration::try_from("20M").unwrap();
        assert_eq!(duration, Duration::from_secs(1200));
    }

    #[test]
    fn test_parse_hours() {
        let duration = Duration::try_from("2h").unwrap();
        assert_eq!(duration, Duration::from_secs(60 * 60 * 2));
    }

    #[test]
    fn test_reject_units() {
        let duration = Duration::try_from("2k");
        assert!(duration.is_err());
    }

    #[test]
    fn test_reject_value() {
        let duration = Duration::try_from("somethingh");
        assert!(duration.is_err());
    }
}
