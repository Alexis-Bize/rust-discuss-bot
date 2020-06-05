#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate futures;
extern crate rusqlite;
extern crate chrono;
extern crate itertools;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;
use itertools::Itertools;

mod models;
mod messages;
mod repository;
mod token;

use models::*;
use messages::*;
use repository::*;

use rocket_contrib::json::{Json, JsonValue};
use rocket::request::LenientForm;
use regex::Regex;

#[get("/")]
fn hello(){
}

#[post("/respond", data = "<data>")]
fn slack_exchange(data: LenientForm<SlackPayload>) {
    println!("{}", data.payload);
    let v: Payload = serde_json::from_str(&data.payload).unwrap();
    let succeed: bool = add_discuss(&v.actions[0].action_id, v.actions[0].selected_options.iter().map(|choice| (&choice.value, &choice.text.text)).collect::<Vec<_>>());

    send_response(succeed, v.response_url);
}

#[post("/", data = "<data>")]
fn slack(data: LenientForm<SlackInput>) -> Json<JsonValue> {
    let slack_input: SlackInput = data.into_inner();
    println!("{:?}", serde_json::to_string(&slack_input));

    let command_add = "add";
    let reg_register = Regex::new(r#"^[R,r]egister (to )*"#).unwrap();
    let reg_unregister = Regex::new(r#"^[U,u]nregister (from )*"#).unwrap();
    let command_topics = "topics";
    let command_mytopics = "mytopics";
    let reg_create_topics = Regex::new(r#"^[C,c]reate (topic(s)? )*"#).unwrap();

    let mut command = slack_input.text;
    let message: Json<JsonValue>;
    if command.starts_with(command_add) {
        command = command.replace(command_add, "");
        let url = get_url_from_command(&command);
        if !url.is_empty() {
            message = get_topics_choice_message(url)
        } else {
            message = get_fail_message("add")
        }
    }
    else if reg_unregister.is_match(&command) {
        if handle_unregister_command(&reg_unregister.replace_all(&command, ""), &slack_input.user_id) {
            let topics = get_user_topics(&slack_input.user_id);
            message = get_topics_message(&topics);
        } else {
            message = get_fail_message("unregister")
        }
    }
    else if reg_register.is_match(&command) {
        if handle_register_command(&reg_register.replace_all(&command, ""), &slack_input.user_id) {
            let topics = get_user_topics(&slack_input.user_id);
            message = get_topics_message(&topics);
        } else {
            message = get_fail_message("register")
        }
    }
    else if command.starts_with(command_mytopics) {
        command = command.replace(command_mytopics, "");
        let topics = get_user_topics(&slack_input.user_id);
        message = get_topics_message(&topics);
    }
    else if command.starts_with(command_topics) {
        command = command.replace(command_topics, "");
        let topics = get_topics(String::new());
        message = get_topics_message(&topics);
    }
    else if reg_create_topics.is_match(&command) {
        handle_create_topic_command(&reg_create_topics.replace_all(&command, ""));
        message = get_help_message();
    }
    else { message = get_help_message() }

    return message;
}

#[post("/topics", data = "<data>")]
fn get_topics_options(data:LenientForm<SlackPayload>) -> Json<JsonValue> {
    println!("{:?}", &data.payload);
    let topic_request_payload: LabelRequestPayload = serde_json::from_str(&data.payload).unwrap();

    let mut topics: Vec<(i32, String)> = get_topics((&topic_request_payload.value).to_string());
    println!("{}", &topics.len());

    if(topics.len() == 0){
        topics.push((-1, topic_request_payload.value));
    }
    get_topics_option_message(topics)
}

fn handle_create_topic_command(command: &str) {
    let reg_comma_separated = Regex::new(r#"([^,]+)"#).unwrap();
    for cap in reg_comma_separated.captures_iter(command) {
        println!("{}",  &cap[0].trim());
        add_topic(&cap[0].trim().to_lowercase());
    }
}

fn get_url_from_command(command: &str) -> String {
    let url_wanted = command.split_whitespace().next().unwrap_or("");

    println!("{}", &url_wanted);
    let url_regex: Regex = Regex::new(r"^(?:http(s)?://)?[\w.-]+(?:\.[\w\.-]+)+([\w\-\._~:/?#\[\]@!\$&'\(\)\*\+,;=.])+$").unwrap();

    if !url_regex.is_match(&url_wanted) {
        println!("not found an url");
        return "".to_string();
    }

    let mut url_found: String = "".to_string();
    for url in url_regex.captures_iter(&url_wanted) {
        url_found = url[0].to_string();
        println!("{}", url_found);
        break;
    }

    return url_found
}

fn handle_register_command(command: &str, user_id: &str) -> bool {
    let reg_comma_separated = Regex::new(r#"([^,]+)"#).unwrap();
    for cap in reg_comma_separated.captures_iter(command) {
        println!("REG : {}",  &cap[0].trim());
        register_user_to_topic(user_id, &cap[0].trim().to_string());
    }
    true
}

fn handle_unregister_command(command: &str, user_id: &str) -> bool {
    let reg_comma_separated = Regex::new(r#"([^,]+)"#).unwrap();
    for cap in reg_comma_separated.captures_iter(command) {
        println!("{}",  &cap[0].trim());
        unregister_user_to_topic(user_id, &cap[0].trim().to_string());
    }
    true
}

fn main() {
    initialize_dataset().unwrap();
    //send_hello_world_message();
    // or a scheduler with a given timezone
    let mut scheduler = Scheduler::new();
    //scheduler.every(1.day()).at("3:30 pm").run(|| send_topics());
    scheduler.every(10.seconds()).run(|| send_topics());
    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    rocket::ignite()
        .mount("/", routes![slack])
        .mount("/", routes![slack_exchange])
        .mount("/", routes![get_topics_options])
        .mount("/", routes![hello])
        .launch();
}

fn send_topics() {
    let results = get_urls_by_topic();
    println!("send");
    for (user, urls) in &results.into_iter().group_by(|result| result.user.id.to_string()) {
        let vec_of_urls = urls.map(|u| u.url).collect::<Vec<models::SqlUrl>>();
        send_discuss_message(&user, &vec_of_urls);
        for url in &vec_of_urls {
            println!("{} {} {}", &user, &url.value, &url.topics[0].name);
            update_user_topic_junction(&user, &url.topics[0].id);
        }
    }
}
