use file_io::UserQuery;

pub mod file_io;
pub mod runner;

fn main() {
    let members_question = UserQuery {
        query_text: String::from("Number of members:"),
    };
    let hourly_rate_question = UserQuery {
        query_text: String::from("Average hourly rate per person:")
    };
    let duration_question = UserQuery{
        query_text: String::from("Duration in minutes:")
    };

    let members: u32 = members_question.query_and_parse_as_u32();
    let duration_in_minutes: u32 = duration_question.query_and_parse_as_u32();
    let average_hourly_rate: f64 = hourly_rate_question.query_and_parse_as_f64();

    let config = MeetingConfig {
        members,
        duration_in_minutes,
        average_hourly_rate
    };
    
    runner::run_value_adder(config);

    
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
        if members <= 1 {return Err("Members must be two or greater");}
        if average_hourly_rate <= 0.0 {return Err("Hourly rate must be positive");}

        let config = MeetingConfig {members, duration_in_minutes, average_hourly_rate};

        Ok(config)
    }
}