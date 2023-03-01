use dotenv::dotenv;

use std::env;
use std::io::Error;

mod math;
mod particle;
mod pixelblaze;
mod theme;
use particle::events;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token =
        env::var("PARTICLE_TOKEN").expect("Please set PARTICLE_TOKEN to particle access token.");
    env::var("PIXELBLAZE_HOST").expect("Please set PIXELBLAZE_HOST to http://<ip>.");
    events(token).await;
    Ok(())
}
