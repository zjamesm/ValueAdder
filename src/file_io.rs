use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, io, path::Path};

const STATE_FILENAME: &'static str = "value_adder_info.json";

// following for the LifetimeValue struct is largely from:
// https://stackoverflow.com/questions/72755833/saving-changed-variables-between-runtime-rust
// adapted for my use case

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LifetimeValue {
    value: f64,
}

// as mentioned in the above se link this is a lossy format, but that does not really matter
// though later implementations may utilize a direct conversion to Vec(u8) which should be lossless?

impl LifetimeValue {
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

impl Default for LifetimeValue {
    fn default() -> Self {
        Self { value: 0 as f64}
    }
}


// surely I could do this better, but I dont know how
// query and parse as methods in particular feel bad,
// but suit the use case so idk

pub struct UserQuery {
    pub query_text: String,
}

impl UserQuery {
    fn query(&self) -> String {
        let mut user_input: String = String::new();

        println!("{}", self.query_text);

        io::stdin() 
            .read_line(&mut user_input)
            .expect("Value input failed");

        user_input
    }

    // fn requery(&self, requery_message: &str) -> String {
    //     println!("{}", requery_message);

    //     self.query()
    // }

    pub fn query_and_parse_as_f64(&self) -> f64 {
        let parse_value = self.query();

        let parsed_value: f64 = match parse_value.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("Invalid entry, please submit a valid decimal value");
                self.query_and_parse_as_f64()
            }
        };

        parsed_value
    }

    pub fn query_and_parse_as_u32(&self) -> u32 {
        let parse_value = self.query();

        let parsed_value: u32 = match parse_value.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("Invalid entry, please submit a valid decimal value");
                self.query_and_parse_as_u32()
            }
        };

        parsed_value
    }
}