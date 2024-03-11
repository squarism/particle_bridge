extern crate tera;

use dotenv::dotenv;
use env_logger::Env;

use std::env;
use std::io::Error;

mod config;
mod math;
mod particle;
mod pixelblaze;
use particle::events;

use crate::config::Config;
use clap::Parser;

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
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::default().filter_or("PARTICLE_LOG_LEVEL", "warn");
    env_logger::init_from_env(env);

    dotenv().ok();
    let token =
        env::var("PARTICLE_TOKEN").expect("Please set PARTICLE_TOKEN to particle access token.");

    // get the list of destinations to forward the events to
    // a particle event can go to many pixelblaze boards
    let args = Cli::parse();

    let config = Config::new("config.json.tera".to_owned());

    // main loop of sorts
    events(config, token, args.topic).await;

    Ok(())
}
