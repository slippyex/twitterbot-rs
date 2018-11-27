use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use crate::models::FilterRule;

pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}

pub fn write_file(content: &str, filepath: &str) {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why)
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}


pub fn get_filters_from_storage() -> Vec<FilterRule> {
    let file_content: String = read_file("./filter_rules.json");
    match serde_json::from_str(&file_content) {
        Ok(filters_converted) => filters_converted,
        Err(_err) => Vec::new()
    }
}

pub fn persist_filters_to_storage(filters: Vec<FilterRule>) {
    let content_str = serde_json::to_string_pretty(&filters).unwrap();
    write_file(content_str.as_str(), "./filter_rules.json");
}
