#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod models;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use crate::models::FilterRule;

use rocket_contrib::Json;
use std::path::Path;


fn read_file(filepath: &str) -> String {
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

fn write_file(content: &str, filepath: &str) {
    let path = Path::new(filepath);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why)
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn get_filters_from_storage() -> Vec<FilterRule> {
    let file_content: String = read_file("./filter_rules.json");
    match serde_json::from_str(&file_content) {
        Ok(filters_converted) => filters_converted,
        Err(_err) => Vec::new()
    }

}
#[get("/status")]
fn status() -> String {
    format!("status!")
}

#[get("/filters")]
fn get_filters() -> Json<Vec<FilterRule>> {
    let filters = get_filters_from_storage();
    Json(filters)
}

#[post("/filters", format = "application/json", data = "<filter>")]
fn post_new_filter(filter: Json<FilterRule>) -> Json<Vec<FilterRule>> {
    let mut filters: Vec<FilterRule> = get_filters_from_storage();
    filters.push(filter.into_inner());
    let content_str = serde_json::to_string_pretty(&filters).unwrap();
    write_file(content_str.as_str(), "./filter_rules.json");
    Json(filters)
}

#[delete("/filters")]
fn delete_filter() -> String {
    format!("delete filter!")
}


fn main() {
    rocket::ignite()
        .mount("/", routes![status, get_filters, post_new_filter, delete_filter])
        .launch();
}
