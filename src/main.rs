use std::env;

use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
// use serenity::model::id::GuildId;
use serenity::prelude::*;

macro_rules! modules {
    ($(mod $name:ident;)*) => {
        $(
            mod $name;
            #[allow(unused_imports)]
            use $name::*;
        )*
    };
}

modules! {
    mod commands;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "modal" => {
                    commands::modal::run(&ctx, &command).await.unwrap();
                    None
                }
                "quiz" => {
                    commands::dropdown::run(&ctx, &command).await.unwrap();
                    None
                }
                "givemeanimage" => {
                    commands::givemeanimage::run(&ctx, &command).await.unwrap();
                    None
                }
                "admin" => {
                    commands::admin::run(&ctx, &command).await.unwrap();
                    None
                }
                "gifs" => {
                    commands::gifs::run(&ctx, &command).await.unwrap();
                    None
                }
                "save" => {
                    commands::save::run(&ctx, &command).await;
                    None
                }
                _ => Some("not implemented :/".to_string()),
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
        // ctx.reset_presence();
        ctx.set_presence(Some(ActivityData::custom("miaou ?")), serenity::all::OnlineStatus::DoNotDisturb);
        // ctx.set_presence(Some(ActivityData::competing("24h du Rust")), serenity::all::OnlineStatus::DoNotDisturb);
        // ctx.dnd();

        // let guild_id = GuildId::new(
        //     env::var("GUILD_ID")
        //         .expect("Expected GUILD_ID in environment")
        //         .parse()
        //         .expect("GUILD_ID must be an integer"),
        // );

        // let commands = guild_id
        //     .set_commands(
        //         &ctx.http,
        //         vec![commands::ping::register(), commands::modal::register()],
        //     )
        //     .await;

        // println!("I now have the following guild slash commands: {commands:#?}");

        let _commands =
            // Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                // .await;
            Command::set_global_commands(&ctx.http, 
                vec![
                    commands::ping::register(),
                    commands::modal::register(),
                    commands::dropdown::register(),
                    commands::admin::register(),
                    commands::givemeanimage::register(),
                    commands::gifs::register(),
                    commands::save::register()
                    ],).await;
        println!("Slash commands created.");
        println!("I created the following global slash command: {_commands:#?}");
        // let _context_commands =
        //     // Command::create_global_command(&ctx.http, commands::wonderful_command::register())
        //         // .await;
        //     CreateCommand::contexts(&ctx.http, 
        //         vec![
        //             contexts::pingme::register()
        //             ],).await;
        // println!("Context commands created.")
    }
}

#[tokio::main]
async fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;
    
    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Error creating client");
    // serenity::builder::EditProfile::avatar(client,avatar);
    // client.cache.current_user().edit(cache_http, builder)
    // client.http.edit_profile();
    
    

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
