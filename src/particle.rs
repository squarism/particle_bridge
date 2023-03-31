use eventsource::reqwest::Client;
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
        let sse_event = event.unwrap();
        let data = sse_event.data;
        let topic = &sse_event.event_type;

        match topic {
            None => (),
            Some(_topic) => {
                let theme_message: ThemeData = serde_json::from_str(&data).unwrap();

                // TODO: look at this middle-man
                let hosts = &config.hosts();
                let hosts = hosts.as_array().expect("need an array of hosts");

                forward(hosts, theme_message.data).await;
            }
        }
    }
}
