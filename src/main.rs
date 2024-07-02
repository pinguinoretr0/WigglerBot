use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

mod cmds;

struct Handler;

// sever-id
const GUILD_ID: u64 = 0;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(cmds::ping::run(&command.data.options())),
                "id" => Some(cmds::id::run(&command.data.options())),
                //"attachmentinput" => Some(cmds::attachmentinput::run(&command.data.options())),
                //"modal" => {
                //    cmds::modal::run(&ctx, &command).await.unwrap();
                //    None
                //},
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(GUILD_ID);

        // add commands to the viewable list
        let commands = guild_id
            .set_commands(&ctx.http, vec![
                cmds::ping::register(),
                cmds::id::register(),
                //cmds::welcome::register(),
                //cmds::numberinput::register(),
                //cmds::attachmentinput::register(),
                //cmds::modal::register(),
            ])
            .await;

        // build & display command information
        println!("Built Slash Command: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, cmds::wonderful_command::register())
                .await;

        println!("The following Slash Command has been registered: {guild_command:#?}");
    }
}

// main
#[tokio::main]
async fn main() {
    // the bots token goes here
    const WIGGLER_TOKEN: &str = "";

    // intents to notify the application about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // creates an instance of the client (log in as bot)
    let mut client =
        Client::builder(&WIGGLER_TOKEN, intents).event_handler(Handler).await.expect("Err creating client");

    // starts a shard that listens to events 
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
