mod config;
use config::Config;
use serenity::{
    prelude::*,
    model::prelude::*,
    Client,
};

struct Handler;
impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }
}

fn main() {
    let _ = Config::new().save();
    let config = Config::load().unwrap();
    let mut client = Client::new(config.token(), Handler)
        .expect("Couldn't create the new client!");
    if let Err(why) = client.start() {
        println!("Client error: {}", why)
    }
}