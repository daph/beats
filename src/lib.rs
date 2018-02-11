//! # Beats 0.1.0
//!
//! Swatch internet time (.beats) crate for rust.
//!
//!

extern crate chrono;

use std::fmt;
use chrono::prelude::*;

/// Struct for representing a .beat
#[derive(Debug, PartialEq)]
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
    /// let beat_now = Beat::now();
    /// println!("It is currently: {}", beat_now)
    /// ~~~
    pub fn now() -> Beat {
        let now: DateTime<Utc> = Utc::now();
        calculate_beats(now)
    }

    /// Create a Beat from a [Chrono](https://docs.rs/chrono) DateTime.
    pub fn from_dt<Tz: TimeZone>(time: DateTime<Tz>) -> Beat {
        calculate_beats(time)
    }
}

impl fmt::Display for Beat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{:.3}", self.trunc + self.fract)
    }
}

fn calculate_beats<T: TimeZone>(time: DateTime<T>) -> Beat {
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
}
