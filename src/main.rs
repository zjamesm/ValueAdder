use file_io::UserQuery;

pub mod file_io;
mod runner;

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

    let member_count: f64 = first_question.query_and_parse_as_f64();
    let average_hourly_rate: f64 = second_question.query_and_parse_as_f64();
    let duration_in_minutes: u32 = third_question.query_and_parse_as_u32();

    let duration_in_intervals: u32 = &duration_in_minutes * &MINUTE_IN_SECONDS / &INTERVAL_IN_SECONDS;

    let hourly_cost: f64 = average_hourly_rate * member_count;

    
}