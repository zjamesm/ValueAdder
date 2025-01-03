use serde::de::value::Error;

use crate::MeetingConfig;
use std::error;

const INTERVALS_PER_MINUTE: u32 = 6;

pub fn run(config: MeetingConfig) -> () {
    let interval_count = &config.duration_in_minutes * &INTERVALS_PER_MINUTE;

    





}

fn display_count_up(interval_count: u32) -> Result<(), Error> {
    let meeting_value:u32;

    for t in 0..interval_count {

    }
}