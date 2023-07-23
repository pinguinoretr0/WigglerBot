mod cmds;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

// Commands:
use crate::cmds::commands::*;

#[group]
#[commands(product, quotient, sum, echo)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
		const DISCORD_TOKEN: &str = "BOT_TOKEN";

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";")) // Set bot prefix
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&DISCORD_TOKEN, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
