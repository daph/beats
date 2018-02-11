extern crate chrono;

use std::fmt;
use chrono::prelude::*;

pub struct Beat {
    trunc: f64,
    fract: f64
}

impl Beat {
    pub fn now() -> Beat {
        let now: DateTime<Utc> = Utc::now();
        calculate_beats(now)
    }

    pub fn from<T: TimeZone>(time: DateTime<T>) -> Beat {
        calculate_beats(time)
    }
}

impl fmt::Display for Beat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3}", self.trunc + self.fract)
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
