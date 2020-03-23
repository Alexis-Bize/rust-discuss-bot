use rocket_contrib::json::{Json, JsonValue};
use reqwest::blocking::Client;

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

pub fn get_topics_message(_topics: Vec<(u32,String)>) -> Json<JsonValue> {
    Json(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "topics"
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

pub fn send_response(succeed: bool, response_url: String){
    let client = Client::new();
    client.post(&response_url)
        .header("content-Type", "application/json")
        .header("authorization", "Bearer {your token here}")
        .body(r#"{
                "replace_original": "true",
                "text": "Thanks for your request, we'll process it and get back to you.",
                "response_type": "ephemeral"
            }"#)
        .send()
        .unwrap();
}

// fn send_hello_world_message(){
//     let client = Client::new();
//     client.post("https://slack.com/api/chat.postMessage")
//         .header("content-Type", "application/json")
//         .header("authorization", "Bearer {your token here}")
//         .body("{
//             \"as_user\": true,
//             \"channel\": \"UVBSUR8UX\",
//             \"text\": \"Hello, world\"
//             }")
//         .send()
//         .unwrap();
// }
