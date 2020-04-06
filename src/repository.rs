use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use crate::models;

pub fn initialize_dataset() -> Result<()> {
    let conn = Connection::open("discuss.db")?;

    conn.execute(
        "create table if not exists discuss (
            id integer primary key,
            name text not null unique
        )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists topics (
            id integer primary key,
            name text not null unique
        )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists urls (
            id integer primary key,
            url text not null
        )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists url_topic_junction (
            url_id integer not null references urls(id),
            topic_id integer not null references topics(id),
            added_date date not null
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "create table if not exists user_topics_junction (
            user_id text not null,
            topic_id integer not null references topics(id),
            send_date date
        )",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn get_topics(starting_with:String) -> Vec<(u32, String)> {
    let conn = Connection::open("discuss.db").unwrap();
    let like_choice_stmt =
        format!("SELECT t.id, t.name from topics t
        WHERE t.name LIKE '{}%'",&starting_with);
    let mut stmt = conn.prepare(&like_choice_stmt).unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();

    let mut topics: Vec<(u32, String)> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        topics.push((row.get(0).unwrap(), row.get(1).unwrap()));
    }
    topics
}

pub fn add_discuss(url: &String, topics_id: Vec<&String>) -> bool{
    let conn = Connection::open("discuss.db").unwrap();
    conn.execute(
        "INSERT INTO urls (url) values (?1)",
        &[url],
    ).unwrap();

    let last_id: String = conn.last_insert_rowid().to_string();
    println!("{}", last_id);

    for topic_id in topics_id {
        conn.execute(
            "INSERT INTO url_topic_junction (url_id, topic_id, added_date) values (?1, ?2, datetime('now'))",
            &[&last_id, &topic_id],
        ).unwrap();
    }
    true
}

pub fn add_topic(topic: &str){
    let conn = Connection::open("discuss.db").unwrap();
    conn.execute(
        "INSERT INTO topics (name)
        SELECT ?1
        WHERE NOT EXISTS (SELECT 1 FROM topics WHERE name = ?1)",
        &[topic],
    ).unwrap();
}

pub fn register_user_to_topic(user: &str, topic: &str) {
    let conn = Connection::open("discuss.db").unwrap();
    conn.execute(
        "INSERT INTO user_topics_junction (user_id, topic_id, send_date)
        SELECT ?1, t.id, datetime('now')
        FROM topics t WHERE name = ?2",
        &[user, topic],
    ).unwrap();
}

pub fn get_urls_by_topic() -> Vec<models::SqlUserUrlResult>{
    let conn = Connection::open("discuss.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT user_j.user_id, u.id, u.url, t.id, t.name FROM user_topics_junction user_j
        INNER JOIN url_topic_junction url_j
        INNER JOIN urls u
        INNER JOIN topics t
        WHERE url_j.topic_id = user_j.topic_id
        AND u.id = url_j.url_id
        AND t.id = url_j.topic_id
        AND url_j.added_date > user_j.send_date"
    ).unwrap();

    let mut urls: Vec<models::SqlUserUrlResult> = Vec::new();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let result = models::SqlUserUrlResult {
            user: models::SqlUser {
                id: row.get(0).unwrap()
            },
            url: models::SqlUrl {
                id: row.get(1).unwrap(),
                value: row.get(2).unwrap(),
                topics: vec![models::SqlTopic {
                    id: row.get(3).unwrap(),
                    name: row.get(4).unwrap()
                }]
            }
        };
        urls.push(result);
    }
    urls
}

pub fn update_user_topic_junction(user_id: &String, topic: &u32) {
    let conn = Connection::open("discuss.db").unwrap();
    conn.execute("UPDATE user_topics_junction SET send_date = datetime('now') WHERE user_id = ?1 AND topic_id = ?2", &[user_id, &topic.to_string()]).unwrap();
}
