#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate clokwerk;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate passwords;

#[macro_use]
extern crate magic_crypt;


use std::time::Duration;

use clokwerk::{Scheduler, TimeUnits};
use rocket_contrib::{Json, Value};

use crate::models::FilterRule;

mod bot;
mod storage;
mod models;

#[get("/status")]
fn status() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[get("/filters")]
fn get_filters() -> Json<Vec<FilterRule>> {
    let filters = storage::get_filters_from_storage();
    Json(filters)
}

#[post("/filters", format = "application/json", data = "<filter>")]
fn post_new_filter(filter: Json<FilterRule>) -> Json<Vec<FilterRule>> {
    let mut filters: Vec<FilterRule> = storage::get_filters_from_storage();
    let mut filter_obj = filter.into_inner();
    filter_obj.id = match filters
        .iter()
        .max_by_key(|p| p.id) {
        Some(obj) => Some(obj.id.unwrap() + 1),
        None => Some(1)
    };
    filters.push(filter_obj);
    storage::persist_filters_to_storage(filters.to_vec());
    Json(filters)
}

#[put("/filters/<id>", format = "application/json", data = "<filter>")]
fn update_existing_filter(id: u8, filter: Json<FilterRule>) -> Json<Value> {
    let mut filters = storage::get_filters_from_storage();
    // check, if we already had that filter (update)
    let found = filters
        .iter()
        .find(|p| p.id.unwrap() == id);
    let result = match found {
        Some(_) => {
            delete_filter(id);
            filters = storage::get_filters_from_storage();
            let mut filter_obj = filter.into_inner();
            filter_obj.id = Some(id);
            filters.push(filter_obj);
            storage::persist_filters_to_storage(filters);
            json!({"updated": true})
        },
        None => json!({"updated": false})
    };
    Json(result)
}

#[delete("/filters/<id>")]
fn delete_filter(id: u8) -> Json<Vec<FilterRule>> {
    let mut filters: Vec<FilterRule> = storage::get_filters_from_storage();
    filters = filters
        .into_iter()
        .filter(|i|i.id.unwrap() != id)
        .collect::<Vec<_>>();
    storage::persist_filters_to_storage(filters.to_vec());
    Json(filters)
}


fn main() {
    // checks sanity of bot configuration
    // and, if necessary, creates missing entries
    bot::check_config();

    let mut scheduler = Scheduler::new();
    scheduler.every(10.seconds()).run(|| bot::bot_invocation());
    let thread_handle = scheduler.watch_thread(Duration::from_millis(1000));

    rocket::ignite()
        .mount("/", routes![
            status,
            post_new_filter,
            get_filters,
            update_existing_filter,
            delete_filter
        ])
        .launch();

    thread_handle.stop();
}
