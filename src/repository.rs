use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

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
            topic_id integer not null references topics(id)
        )",
        NO_PARAMS,
    )?;

    // conn.execute(
    //     "INSERT INTO topics (name) values ('Choice 1')",
    //     NO_PARAMS,
    // )?;
    // conn.execute(
    //     "INSERT INTO topics (name) values ('Choice 2')",
    //     NO_PARAMS,
    // )?;
    // conn.execute(
    //     "INSERT INTO topics (name) values ('Choice 3')",
    //     NO_PARAMS,
    // )?;
    Ok(())
}

pub fn get_all_topics(topic_name:String) -> Vec<(u32, String)> {
    let conn = Connection::open("discuss.db").unwrap();
    let like_choice_stmt =
        format!("SELECT t.id, t.name from topics t
        WHERE t.name LIKE '{}%'",&topic_name);
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
            "INSERT INTO url_topic_junction (url_id, topic_id) values (?1, ?2)",
            &[&last_id, &topic_id],
        ).unwrap();
    }
    true
}

pub fn add_topic(topic: String){
    let conn = Connection::open("discuss.db").unwrap();
    conn.execute(
        "INSERT INTO topics (name)
        SELECT ?1
        WHERE NOT EXISTS (SELECT 1 FROM topics WHERE name = ?1)",
        &[topic],
    ).unwrap();
}
