#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate futures;
extern crate rusqlite;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

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

    let succeed: bool = add_discuss(&v.actions[0].action_id, v.actions[0].selected_options.iter().map(|choice| &choice.value).collect::<Vec<_>>());

    send_response(succeed, v.response_url);
}

#[post("/", data = "<data>")]
fn slack(data: LenientForm<SlackInput>) -> Json<JsonValue> {
    let slack_input: SlackInput = data.into_inner();
    println!("{:?}", serde_json::to_string(&slack_input));

    let command_add = "add";
    let command_register = "register";
    let command_topics = "topics";
    let command_create_topics = "create topics";

    let mut command = slack_input.text;

    if command.starts_with(command_add) {
        command = command.replace(command_add, "");
        let url = get_url_from_command(&command);
        return if !url.is_empty() { get_topics_choice_message(url) } else { get_fail_message("add") }
    }
    else if command.starts_with(command_register) {
        command = command.replace(command_register, "");
        return if handle_register_command(&command) { get_success_message("register") } else { get_fail_message("register") } }
    else if command.starts_with(command_topics) {
        command = command.replace(command_topics, "");
        let topics = get_all_topics(String::new());
        return get_topics_message(topics);
    }
    else if command.starts_with(command_create_topics) {
        command = command.replace(command_create_topics, "");
        handle_create_topic_command(&command);
        return get_help_message();
    }
    else { return get_help_message() }
}

#[post("/topics", data = "<data>")]
fn get_topics_options(data:LenientForm<SlackPayload>) -> Json<JsonValue> {
    println!("{:?}", &data.payload);
    let topic_request_payload: LabelRequestPayload = serde_json::from_str(&data.payload).unwrap();

    let mut message =json!({ "options" : []});
    let options = message["options"].as_array_mut().unwrap();
    let topics = get_all_topics(topic_request_payload.value);
    for topic in topics {
        let s = format!("{}{}{}{}{}", r#"{"text": {
                                                "type": "plain_text",
                                                "text": ""#, &topic.1.to_string(), r#"",
                                                "emoji": true
                                            },
                                            "value": ""#, &topic.0.to_string(), r#""
                                        }"#);
        let value: serde_json::Value = serde_json::from_str(&s).unwrap();
        options.push(value)
    }

    Json(message)
}

fn handle_create_topic_command(command: &str) {
    let topics = command.split(',').collect::<Vec<_>>();
    for topic in topics {
        let lower_topic = topic.to_string().trim().to_lowercase();
        add_topic(lower_topic);
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

fn handle_register_command(_command: &str) -> bool {
    true
}

fn main() {
    initialize_dataset().unwrap();

    rocket::ignite()
        .mount("/", routes![slack])
        .mount("/", routes![slack_exchange])
        .mount("/", routes![get_topics_options])
        .mount("/", routes![hello])
        .launch();
}
