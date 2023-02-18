use dotenv::dotenv;
use eventsource::reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    event: String,
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let token = env::var("TOKEN").expect("Please set TOKEN to particle access token.");

    let _result = events(token).await;

    Ok(())
}

// event: squarism/blinkwon
// data: {"data":"{\"theme\":\"off\"}","ttl":60,"published_at":"2023-02-17T23:11:26.574Z","coreid":"api"}
async fn events(token: String) {
    let s = format!("https://api.particle.io/v1/events/squarism%2Fblinkwon?access_token={token}");
    let url = reqwest::Url::parse(&s).unwrap();

    let client = Client::new(url);

    for event in client {
        println!("{}", event.unwrap());
    }
}
