use eventsource::reqwest::Client;
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;
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

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeMessage {
    pub theme: String,
    pub brightness: Option<u8>,
}

pub async fn events(config: Config, token: String, topic: String) {
    let s = format!("https://api.particle.io/v1/events/{topic}?access_token={token}");
    let url = reqwest::Url::parse(&s).unwrap();

    let client = Client::new(url);

    // return client and move this to main
    for event in client {
        // log and continue
        if event.is_err() {
            warn!("Problem with SSE Event");
        } else {
            let topic = sse_event_topic(&event);

            match topic {
                None => (),
                Some(_topic) => {
                    let data = sse_event_data(&event);
                    let theme_message: ThemeData = serde_json::from_str(&data).unwrap();

                    // TODO: look at this middle-man
                    let hosts = &config.hosts();
                    let hosts = hosts.as_array().expect("need an array of hosts");

                    forward(hosts, theme_message.data).await;
                }
            }
        }
    }
}

// to avoid inline and borrow checker problems
fn sse_event_data(
    event: &Result<eventsource::event::Event, eventsource::reqwest::Error>,
) -> String {
    event.as_ref().ok().map(|e| e.data.to_string()).unwrap()
}

// to avoid inline and borrow checker problems
fn sse_event_topic(
    event: &Result<eventsource::event::Event, eventsource::reqwest::Error>,
) -> Option<String> {
    event
        .as_ref()
        .ok()
        .and_then(|e| e.event_type.as_ref().map(|s| s.to_string()))
}
