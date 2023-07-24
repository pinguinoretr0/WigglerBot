mod cmds;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

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
		const DISCORD_TOKEN: &str = "NzQ3MTEzNDEyNzIyNjg4MDgw.GgHESt.sFcwIBKG93Ly3ld1zx5u1O2N8CYEpHi_etaai4";

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";") // set box prefix
									 .allow_dm(true) // allows bot to dm
									 .case_insensitivity(false)) // might change this
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&DISCORD_TOKEN, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // error checking
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
