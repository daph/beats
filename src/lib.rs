//! # Beats
//!
//! Swatch internet time (.beats) crate for rust.
//!

extern crate chrono;

use std::fmt;
use std::cmp::Ordering;
use chrono::prelude::*;

/// Struct for representing a .beat
#[derive(Debug)]
pub struct Beat {
    trunc: f64,
    fract: f64
}

impl Beat {
    /// Create a Beat for the current time
    ///
    /// # Example
    /// ~~~
    /// use beats::Beat;
    ///
    /// let now = Beat::now();
    /// println!("It is currently: {}", now);
    /// ~~~
    pub fn now() -> Beat {
        Beat::from(Utc::now())
    }
}

impl fmt::Display for Beat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{:07.3}", self.trunc + self.fract)
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for Beat {
    fn from(time: DateTime<Tz>) -> Beat {
        calculate_beats(time)
    }
}

impl PartialEq for Beat {
    fn eq(&self, other: &Beat) -> bool {
        self.trunc == other.trunc && self.fract == other.fract
    }
}

impl PartialOrd for Beat {
    fn partial_cmp(&self, other: &Beat) -> Option<Ordering> {
        let my_time = &self.trunc + &self.fract;
        let other_time = &other.trunc + &other.fract;
        match (my_time <= other_time, my_time >= other_time) {
            (false, false) => None,
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
            (true, true) => Some(Ordering::Equal),
        }
    }
}

fn calculate_beats<T: TimeZone>(time: DateTime<T>) -> Beat {
    // Correct the timezone to UTC because beats are the same time everywhere
    let time = time.with_timezone(&Utc);
    let offset = time.second() + ((time.minute() * 60) + ((time.hour() + 1) * 3600));
    let beats = offset as f64 / 86.4;
    let trunc = beats.trunc();
    let fract = beats.fract();

    wrap(Beat { trunc, fract })
}

fn wrap(beat: Beat) -> Beat {
    let trunc = if beat.trunc >= 1000.0 {
        beat.trunc - 1000.0
    } else {
        beat.trunc
    };

    Beat { trunc, fract: beat.fract }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_wrap() {
        let beat = Beat {
            trunc: 1000.0,
            fract: 0.001
        };

        assert_eq!(wrap(beat), Beat {trunc: 0.0, fract: 0.001});
    }

    #[test]
    fn from_time() {
        assert_eq!(Beat::from(Utc::now()), Beat::now());
        assert_eq!(Beat::from(Local::now()), Beat::now());
    }

    #[test]
    fn ordering() {
        let beat1 = Beat {
            trunc: 123.0,
            fract: 0.456
        };

        let beat2 = Beat {
            trunc: 456.0,
            fract: 0.123
        };

        assert!(beat1 < beat2);
        assert!(beat2 > beat1);
    }

    #[test]
    fn display() {
        let beat_nopad = Beat {
            trunc: 123.0,
            fract: 0.123
        };

        let beat_leftpad = Beat {
            trunc: 23.0,
            fract: 0.123
        };

        let beat_rightpad = Beat {
            trunc: 123.0,
            fract: 0.12
        };

        let beat_bothpad = Beat {
            trunc: 23.0,
            fract: 0.12
        };

        assert_eq!(format!("{}", beat_nopad), "@123.123");
        assert_eq!(format!("{}", beat_leftpad), "@023.123");
        assert_eq!(format!("{}", beat_rightpad), "@123.120");
        assert_eq!(format!("{}", beat_bothpad), "@023.120");
    }
}
