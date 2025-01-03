use file_io::UserQuery;

pub mod file_io;
pub mod runner;

const INTERVAL_IN_SECONDS: u32 = 10;
const HOUR_IN_SECONDS: u32 = 3600;
const MINUTE_IN_SECONDS: u32 = 60;

fn main() {
    let first_question = UserQuery {
        query_text: String::from("Number of members:"),
    };
    let second_question = UserQuery {
        query_text: String::from("Average hourly rate per person:")
    };
    let third_question = UserQuery{
        query_text: String::from("Duration in minutes:")
    };

    let members: u32 = first_question.query_and_parse_as_u32();
    let duration_in_minutes: u32 = second_question.query_and_parse_as_u32();
    let average_hourly_rate: f64 = third_question.query_and_parse_as_f64();

    let config = MeetingConfig {
        members,
        duration_in_minutes,
        average_hourly_rate
    };


}

pub struct MeetingConfig {
    members: u32,
    duration_in_minutes: u32,
    average_hourly_rate: f64
}

impl MeetingConfig {
    pub fn build(
        members: u32,
        duration_in_minutes: u32,
        average_hourly_rate: f64
    ) -> 
    Result<MeetingConfig, &'static str> {
        if members <= 1 {return Err("Members must be greater than 1");}
        if duration_in_minutes <= 0 {return Err("Duration must be postive");}
        if average_hourly_rate <= 0.0 {return Err("Hourly rate must be positive");}

        let config = MeetingConfig {members, duration_in_minutes, average_hourly_rate};

        Ok(config)
    }

    
}