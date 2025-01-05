use std::{thread, time::Duration};

use crate::MeetingConfig;

const INTERVALS_PER_MINUTE: u32 = 6;
const INTERVAL_LENGTH_IN_SECONDS: f64 = 10 as f64;
const HOUR_IN_SECONDS: f64 = 3600 as f64;

pub fn run(config: MeetingConfig) -> f64 {
    let interval_count = &config.duration_in_minutes * &INTERVALS_PER_MINUTE;
    let value_per_interval = config.members as f64 * &config.average_hourly_rate / HOUR_IN_SECONDS * INTERVAL_LENGTH_IN_SECONDS;

    let meeting_value: f64 = display_count_up(interval_count, value_per_interval);

    println!("Wow! ${:.2} is a lot of value, great work!", &meeting_value);

    meeting_value
}

fn display_count_up(interval_count: u32, value_per_interval: f64) -> f64 {
    let mut meeting_value: f64 = 0.0;

    println!("Initializing count-up...");

    for _ in 0..interval_count {
        meeting_value += value_per_interval;
        thread::sleep(Duration::from_secs_f64(INTERVAL_LENGTH_IN_SECONDS));
        println!("Value added so far: ${:.2}", meeting_value);
    };

    meeting_value
}