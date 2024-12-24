use file_io::UserQuery;

pub mod file_io;
mod runner;

fn main() {
    const INTERVAL_IN_SECONDS: u32 = 10;
    const HOUR_IN_SECONDS: u32 = 3600;

    let first_question = UserQuery {
        query_text: String::from("Number of members:"),
    };
    let second_question = UserQuery {
        query_text: String::from("Average hourly rate per person:")
    };
    let third_question = UserQuery{
        query_text: String::from("Duration in minutes:")
    };

    let member_count: u32 = first_question.query_and_parse_as_u32();
    let average_hourly_rate: f64 = second_question.query_and_parse_as_f64();
    let duration: u32 = third_question.query_and_parse_as_u32();


}