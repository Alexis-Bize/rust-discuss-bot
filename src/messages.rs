use rocket_contrib::json::{Json, JsonValue};
use reqwest::blocking::Client;
use crate::token::*;
use crate::models;

pub fn get_fail_message(kind: &str)-> Json<JsonValue> {
    Json(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": format!("fail: {}", kind)
                }
            }
        ]}))
}

pub fn get_success_message(kind: &str) -> Json<JsonValue>{
    Json(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": format!("success: {}", kind)
                }
            }
        ]}))
}

pub fn get_help_message() -> Json<JsonValue> {
    Json(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hey there 👋 I'm Discuss Bot. I'm here to help you manage your technologie watch in Slack.\nThere are two main command to know:"
                }
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*1️⃣ Use the `/discuss add {url} topics {topic} ` command*. Type `/discuss add` followed by the {url} you want to add to the discuss list. You should also provide a list of topic comma separated to set topic to your link (e.g. `topics \" topic1, topic2, topic3\"`) with a *maximum of 3 topics*."
                }
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*2️⃣ Use the `/discuss register {topic}` command.* This will register you as user to topics and enable me, DiscussBot, to give you a daily summary of article you may be interested on.."
                }
            },
            {
                "type": "divider"
            },
            {
                "type": "context",
                "elements": [
                    {
                        "type": "mrkdwn",
                        "text": "👀 View all topics with `/discuss topics`"
                    }
                ]
            }
        ]
    }))
}

pub fn get_topics_choice_message(url: String) -> Json<JsonValue> {

    let message =json!({
        "blocks": [
            {
                "type": "section",
                "block_id": "section678",
                "text": {
                    "type": "mrkdwn",
                    "text": "Pick one or more topics from the list (max 3)"
                },
                "accessory": {
                    "action_id": url,
                    "type": "multi_external_select",
                    "placeholder": {
                        "type": "plain_text",
                        "text": "Select topics",
                        "emoji": true
                    },
                    "max_selected_items": 3
                }
            }
        ]
    });

    Json(message)
}

pub fn get_topics_option_message(topics: Vec<(u32, String)>) -> Json<JsonValue> {
    let mut message =json!({ "options" : []});
    let options = message["options"].as_array_mut().unwrap();
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

pub fn get_topics_message(topics: &Vec<(u32, String)>) -> Json<JsonValue> {
    let mut message =json!({ "blocks" : []});
    let options = message["blocks"].as_array_mut().unwrap();
    for topic in topics {
        let s = format!("{}{}{}", r#"{  "type": "section",
                                            "text": {
                                                "type": "plain_text",
                                                "text": ""#, &topic.1.to_string(), r#""
                                            }
                                        }"#);
        let value: serde_json::Value = serde_json::from_str(&s).unwrap();
        options.push(value)
    }

    Json(message)
}

pub fn send_response(succeed: bool, response_url: String){
    let client = Client::new();
    client.post(&response_url)
        .header("content-Type", "application/json")
        .header("authorization", format!("Bearer {}", get_token()))
        .body(r#"{
                "replace_original": "true",
                "text": "Thanks for your request, we'll process it and get back to you.",
                "response_type": "ephemeral"
            }"#)
        .send()
        .unwrap();
}

pub fn send_discuss_message(user_chan: &String, urls: &Vec<models::SqlUrl>) {
        let client = Client::new();
        let mut message =json!({
            "as_user": true,
            "channel": user_chan,
            "attachments": [ ]
        });

        let attachments = message["attachments"].as_array_mut().unwrap();
        for url in urls {
            let s = format!("{}{}{}{}{}", r#"{  "type": "section",
                                            "text": ""#, url.value,r#"\n`"#, url.topics[0].name,r#"`"
                                        }"#);
            let value: serde_json::Value = serde_json::from_str(&s).unwrap();
            attachments.push(value);
        }
        let message_str = serde_json::to_string(&message).unwrap();
        println!("{}", message_str);
        // let message = format!("{}{}{}", r#"{
        //                                     "as_user": true,
        //                                     "channel": ""#, user_chan, r#"",
        //                                     "attachments": [ { "text": "Hello, world" } ]
        //                                     }
        //                                 }"#);

        client.post("https://slack.com/api/chat.postMessage")
            .header("content-Type", "application/json")
            .header("authorization", format!("Bearer {}", get_token()))
            .body(message_str)
            .send()
            .unwrap();
}

pub fn send_hello_world_message(){
    let client = Client::new();
    let message = format!("{}{}{}", r#"{
                                        "as_user": true,
                                        "channel": ""#, "UVBSUR8UX", r#"",
                                        "attachments": [ { "text": "https://medium.com/androiddevelopers/alter-type-with-typealias-4c03302fbe43" } ]
                                        }
                                    }"#);
    client.post("https://slack.com/api/chat.postMessage")
        .header("content-Type", "application/json")
        .header("authorization", format!("Bearer {}", get_token()))
        .body(message)
        .send()
        .unwrap();
}
