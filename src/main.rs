use clap::{App, Arg};
use reqwest;
use rusqlite::params;
use rusqlite::Connection;
use select::document::Document;
use select::predicate::{Name};
use std::error::Error;
use std::sync::{Arc, Mutex};
use url::Url;

#[tokio::main]
async fn main() {
    let matches = App::new("Lockjaw Spider")
        .version("1.0")
        .author("By Mephistolist")
        .about("Web spider in Rust that hides tracks.")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("Sets the starting URL for the spider")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("spoof")
                .short("s")
                .long("spoof")
                .value_name("Spoofed_IP")
                .help("Sets the spoofed IP for headers")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("DB_FILE")
                .help("Sets the SQLite database file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("user_agent")
                .short("a")
                .long("user-agent")
                .value_name("USER_AGENT")
                .help("Sets the user agent string")
                .takes_value(true)
                .default_value("Lockjaw Spider 1.0"),
        )
        .get_matches();

    let start_url = matches.value_of("url").unwrap_or_default();
    let db_file = matches.value_of("database").unwrap_or_default();
    let spoof_ip = matches.value_of("spoof").unwrap_or_default();
    let user_agent = matches.value_of("user_agent").unwrap_or_default();

    if let Err(err) = run_spider(start_url, db_file, spoof_ip, user_agent).await {
        eprintln!("Error: {}", err);
    }
}

async fn run_spider(start_url: &str, db_file: &str, spoof_ip: &str, user_agent: &str) -> Result<(), Box<dyn Error>> {
    let conn = Arc::new(Mutex::new(Connection::open(db_file)?)); 

    // Create a reqwest Client with a custom user-agent
    let client = reqwest::blocking::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(user_agent).expect("Error creating header"));
            headers.insert("X-Forwarded-For", reqwest::header::HeaderValue::from_str(spoof_ip).expect("Error creating header"));
            headers.insert("X-Originating-IP", reqwest::header::HeaderValue::from_str(spoof_ip).expect("Error creating header"));
            headers.insert("X-Remote-IP", reqwest::header::HeaderValue::from_str(spoof_ip).expect("Error creating header"));
            headers.insert("X-Remote-Addr", reqwest::header::HeaderValue::from_str(spoof_ip).expect("Error creating header"));
            headers
        })
        .build()?;

    create_tables(&conn)?;

    spider(start_url, Arc::new(Mutex::new(Vec::new())), &client, &conn)?;
    Ok(())
}

fn create_tables(conn: &Arc<Mutex<Connection>>) -> Result<(), rusqlite::Error> {
    conn.lock().unwrap().execute(
        "CREATE TABLE IF NOT EXISTS links (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL,
            status_code INTEGER,
            has_form TEXT
        )",
        params![],
    )?;

    Ok(())
}

//fn spider(url: &str, mut visited: Vec<String>, client: &reqwest::blocking::Client, conn: &Arc<Mutex<Connection>>) -> Result<(), Box<dyn Error>> {
fn spider(url: &str, visited: Arc<Mutex<Vec<String>>>, client: &reqwest::blocking::Client, conn: &Arc<Mutex<Connection>>) -> Result<(), Box<dyn Error>> {
    if visited.lock().unwrap().contains(&url.to_string()) {
        return Ok(());
    }

    let response = client.get(url).send()?;

    // Print the URL and status code
    let status_code = response.status().as_u16() as i64;
    println!("{} - Status: {}", url, status_code);

    // Check for the existence of a <form> tag
    let body = response.text()?;
    let document = Document::from(body.as_str());

    let mut has_form = false;
    for _node in document.find(Name("form")) {
        has_form = true;
        break;
    }

    let has_form_str = if has_form { "y" } else { "n" };

    // Insert into the database with information about the <form> tag
    conn.lock().unwrap().execute(
        "INSERT INTO links (url, status_code, has_form) VALUES (?1, ?2, ?3)",
        params![url, status_code, has_form_str],
    )?;

    visited.lock().unwrap().push(url.to_string());

    for node in document.find(Name("a")) {
        if let Some(link) = node.attr("href") {
            // Check if the link is an absolute URL or a relative path
            let absolute_url = if link.starts_with("http://") || link.starts_with("https://") {
                link.to_string()
            } else if link.starts_with('/') {
                let base_url = Url::parse(url)?;
                base_url.join(link)?.to_string()
            } else {
                // Skip relative URLs without a base
                continue;
            };

            spider(&absolute_url, visited.clone(), client, conn)?;
        }
    }

    Ok(())
}
