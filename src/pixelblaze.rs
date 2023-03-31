use futures_util::SinkExt;
use futures_util::StreamExt;
use serde_json::Value;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

use crate::config::Config;
use crate::math::round;
use crate::particle::ThemeMessage;

use log::warn;

pub async fn forward(hosts: &[Value], message: ThemeMessage) {
    for pixelblaze in hosts.iter() {
        // TODO: this means that I need a config struct and not a bunch of serde JSON Values
        let host = pixelblaze
            .get("host")
            .expect("host missing in pixelblaze section")
            .as_str()
            .unwrap();

        // TODO: better error messages
        let url = Url::parse(host).unwrap();

        let result = send_message(&url, pixelblaze, message.clone()).await;
        match result {
            Ok(_) => (),
            Err(_) => warn!("Could not send"),
        }
    }
}

// This function connects to the given URL and sends a message over the WebSocket.
async fn send_message(
    url: &Url,
    pixelblaze: &Value,
    message: ThemeMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    let pixelblaze_message = theme(message.clone(), pixelblaze);
    dbg!(&pixelblaze_message);

    match pixelblaze_message {
        Some(m) => write.send(Message::Text(m)).await?,
        None => warn!("I do not know what the theme `{}` is", message.theme),
    }

    while let Some(msg) = read.next().await {
        match msg {
            Err(e) => {
                panic!("Error on client stream: {e:?}")
            }
            Ok(m) => m,
        };

        // accept a message from the pixelblaze and close the socket?
        // if we don't do it this way (ie: ignore the message) then it
        // seems to lock up.  Not sure on this, really.
        // it definitely sends `{"ack":1}`
        write.close().await.expect("Failed to disconnect");
    }

    Ok(())
}

fn theme(message: ThemeMessage, pixelblaze: &Value) -> Option<String> {
    match message {
        ThemeMessage {
            ref theme,
            brightness: None,
        } if theme == "off" => Some(r#"{ "brightness": 0.0 }"#.to_owned() + "\n"),

        // the physical ambience node control sends 0 with "off"
        // because of C :(
        // https://github.com/squarism/blinkwon/blob/main/ambience_control.ino#L82
        // so we need another matcher here
        ThemeMessage {
            ref theme,
            brightness: Some(0),
        } if theme == "off" => Some(r#"{ "brightness": 0.0 }"#.to_owned() + "\n"),

        ThemeMessage {
            theme,
            brightness: Some(brightness),
        } => {
            let themes = pixelblaze.get("themeIds").unwrap();
            let theme_id = themes.get(&theme);

            match theme_id {
                Some(id) => {
                    // clamp brightness down, pixelblaze strip is brighter
                    let brightness = round(brightness as f32 / 255.0, 3) * 0.80;

                    let c = Config::new("config.json.tera".to_owned());

                    // look up the config and populate the template with variables
                    let theme_definition = c
                        .theme_definition(theme.clone(), brightness, id.to_string())
                        .to_string();
                    Some(theme_definition)
                }
                None => None,
            }
        }

        _ => None,
    }
}

// #[cfg(test)]
// mod tests {
//     use mockall::predicate::*;
//     use mockall::*;
//     use serde_json::json;

//     use super::*;

//     #[tokio::test]
//     async fn test_forward() {
//         let host = json!("ws://1.2.3.4:81");
//         let hosts = vec![host];

//         let mut mock_send_message = Mocksend_message::new();
//         mock_send_message
//             .expect_call()
//             .with(eq(url.clone()), eq(message.clone()))
//             .times(1)
//             .returning(|_, _| Ok(()));

//         let message: ThemeMessage = ThemeMessage {
//             theme: "test-theme".to_owned(),
//             brightness: Some(42),
//         };
//         let tmp_hosts = vec!["no".to_owned()];
//         let result = forward(&hosts, message, tmp_hosts).await;

//         let expected = "no";

//         //     let result = theme_definition("orange".to_owned(), 42.0);
//         //     let expected = "{\"activeProgramId\":\"K58J7fPWpqXjeoLsW\",\"brightness\":42.0,\"setVars\":{\"nCurrentLight\":0}}";
//         assert_eq!(result, expected);
//     }
// }
