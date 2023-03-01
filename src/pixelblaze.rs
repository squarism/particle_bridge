use futures_util::SinkExt;
use futures_util::StreamExt;
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::math::round;
use crate::particle::ThemeMessage;
use crate::theme;

pub async fn forward(message: ThemeMessage) {
    let pixelblaze_host = env::var("PIXELBLAZE_HOST").unwrap();

    let url = url::Url::parse(pixelblaze_host.as_str()).unwrap();
    let (ws_stream, _) = connect_async(&url)
        .await
        .expect("Pixelblaze connect failed");
    let (mut write, mut read) = ws_stream.split();

    let pb_request = theme(message);

    write.send(Message::Text(pb_request)).await.unwrap();

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
}

fn theme(message: ThemeMessage) -> String {
    match message {
        ThemeMessage {
            ref theme,
            brightness: None,
        } if theme == "off" => r#"{ "brightness": 0.0 }"#.to_owned() + "\n",

        // the physical ambience node control sends 0 with "off"
        // because of C :(
        // https://github.com/squarism/blinkwon/blob/main/ambience_control.ino#L82
        // so we need another matcher here
        ThemeMessage {
            ref theme,
            brightness: Some(0),
        } if theme == "off" => r#"{ "brightness": 0.0 }"#.to_owned() + "\n",

        ThemeMessage {
            theme,
            brightness: Some(brightness),
        } => {
            // clamp brightness down, pixelblaze strip is brighter
            let brightness = round(brightness as f32 / 255.0, 3) * 0.80;

            match theme.as_str() {
                "bluegreen" => theme::bluegreen(brightness),
                "orange" => theme::orange(brightness),
                "blacklight" => theme::blacklight(brightness),
                "white" => theme::white(brightness),
                _ => panic!("bad message"),
            }
        }

        _ => {
            println!("something else");
            "".to_owned()
        }
    }
}
