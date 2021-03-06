// #[macro_use] extern crate serde_derive;
// #[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize, FromForm)]
pub struct SlackInput {
    pub token: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub user_id: String,
    pub user_name: String,
    pub team_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub response_url: String,
    pub actions: Vec<Action>,
    pub user: User
}

#[derive(Serialize, Deserialize)]
pub struct LabelRequestPayload {
    pub value: String
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub action_id: String,
    pub selected_options: Vec<Choice>
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub text: Text,
    pub value: String
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub r#type: String,
    pub text: String,
    pub emoji: bool
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct SlackPayload {
    pub payload: String
}

pub struct SqlUser {
    pub id: String
}

pub struct SqlTopic {
    pub id: u32,
    pub name: String
}

pub struct SqlUrl {
    pub id: u32,
    pub value: String,
    pub topics: Vec<SqlTopic>
}

pub struct SqlUserUrlResult {
    pub user: SqlUser,
    pub url: SqlUrl
}
