#[allow(warnings)]
mod parser;

use parser::Guest;

struct Component;

impl Guest for Component {
    fn parse_file(raw_data: Vec<u8>) -> String {
        // Implement the logic to parse the raw_data and return a string
        String::from_utf8(raw_data).unwrap_or_else(|_| "Invalid UTF-8 data".to_string())
    }
}

parser::export!(Component with_types_in parser);
