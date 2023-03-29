extern crate tera;

use dotenv::dotenv;

use std::env;
use std::io::Error;

mod config;
mod math;
mod particle;
mod pixelblaze;
mod theme;
use particle::events;

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[clap(
    version,
    about,
    long_about = "particle_bridge is a daemon to forward particle.io events to pixelblaze boards for LED control and syncing"
)]
struct Cli {
    #[arg(
        long("topic"),
        help("particle.io pubsub topic to listen to"),
        required(true)
    )]
    topic: String,

    #[arg(
        long("pixelblaze_host"),
        num_args(1..),
        action(ArgAction::Append),
        help("multiple values allowed to forward messages to many pixelblaze boards"),
        required(true)
    )]
    pixelblaze_host: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token =
        env::var("PARTICLE_TOKEN").expect("Please set PARTICLE_TOKEN to particle access token.");

    // get the list of destinations to forward the events to
    // a particle event can go to many pixelblaze boards
    let args = Cli::parse();

    // main loop of sorts
    events(token, args.pixelblaze_host, args.topic).await;

    Ok(())
}
