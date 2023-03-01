use eventsource::reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::io::Write;

use crate::pixelblaze::forward;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    event: String,
    data: String,
}

pub fn parse_data_field<'de, D>(deserializer: D) -> Result<ThemeMessage, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let v: Value = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
    serde_json::from_value(v).map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct ThemeData {
    #[serde(deserialize_with = "parse_data_field")]
    pub data: ThemeMessage,
}

#[derive(Debug, Deserialize)]
pub struct ThemeMessage {
    pub theme: String,
    pub brightness: Option<u8>,
}

// event: squarism/blinkwon
// data: {"data":"{\"theme\":\"off\"}","ttl":60,"published_at":"2023-02-17T23:11:26.574Z","coreid":"api"}
// OR
// "{\"data\":\"{\\\"brightness\\\":40,\\\"theme\\\":\\\"bluegreen\\\"}
pub async fn events(token: String) {
    // TODO: unhardcode topic
    let s = format!("https://api.particle.io/v1/events/squarism%2Fblinkwon?access_token={token}");
    let url = reqwest::Url::parse(&s).unwrap();

    let client = Client::new(url);

    for event in client {
        let sse_event = event.unwrap();
        let data = sse_event.data;
        let topic = &sse_event.event_type;

        match topic {
            None => {
                // TODO: do nothing, this is just placeholder
                print!(".");
                // io::stdout().flush().unwrap();
                io::stdout().flush().unwrap();
            }
            Some(_topic) => {
                let theme_message: ThemeData = serde_json::from_str(&data).unwrap();
                forward(theme_message.data).await;
            }
        }
    }
}
