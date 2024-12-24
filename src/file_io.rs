use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, io, path::Path};

const STATE_FILENAME: &'static str = "value_adder_info.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistentState {
    counter: f64,
}

impl PersistentState {
    pub fn load() -> Self {
        let path = Path::new(STATE_FILENAME);

        if path.exists() && path.is_file() {
            let file = File::open(path).unwrap();
            serde_json::from_reader(BufReader::new(file)).unwrap()
        } else {
            Self::default()
        }
    }

    pub fn store(&self) {
        let file = File::create(STATE_FILENAME).unwrap();
        serde_json::to_writer(file, self).unwrap();
    }
}

impl Default for PersistentState {
    fn default() -> Self {
        Self { counter: 0 as f64}
    }
}

pub struct UserQuery {
    pub query_text: String,
}

impl UserQuery {
    fn query(&self) -> String {
        let mut user_input: String = String::new();

        print!("{}", self.query_text);

        io::stdin() 
            .read_line(&mut user_input)
            .expect("Value input failed");

        user_input
    }

    pub fn query_and_parse_as_f64(&self) -> f64 {
        let parse_value = self.query();

        match parse_value.trim().parse() {
            Ok(num) => num,
            Err(_e) => self.query_and_parse_as_f64()
        }
    }

    pub fn query_and_parse_as_u32(&self) -> u32 {
        let parse_value = self.query();

        match parse_value.trim().parse() {
            Ok(num) => num,
            Err(_e) => self.query_and_parse_as_u32()
        }
    }
}