use std::time::Duration;

use serenity::all::{Context, CreateAttachment, CreateButton, CreateCommand, CreateEmbed, EditProfile, Http};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

use serenity::futures::StreamExt;
use serenity::model::prelude::*;
// use tokio::fs::File;

async fn log_system_load(ctx: &Context, interaction: &ComponentInteraction) {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let embed = CreateEmbed::new()
        .title("System Resource Load")
        .field(
            "CPU Load Average",
            format!("{:.2}%", cpu_load.one * 10.0),
            false,
        )
        .field(
            "Memory Usage",
            format!(
                "{:.2} MB Free out of {:.2} MB",
                mem_use.free as f32 / 1000.0,
                mem_use.total as f32 / 1000.0
            ),
            false,
        );
    let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .ephemeral(true),
    ); //CreateMessage::new().embed(embed)
    let message = interaction.create_response(ctx, builder);
    if let Err(why) = message.await {
        eprintln!("Error sending message: {why:?}");
    };
}

// #[cfg(feature = "http")]
async fn update_pp(http: &Http, current_user: &mut CurrentUser) {
    let avatar = CreateAttachment::path("./assets/profilepictures/1.jpg").await.expect("Failed to read image.");
    current_user.edit(http, EditProfile::new().avatar(&avatar)).await.unwrap();
}

fn button(id: &str, name: &str, emoji: ReactionType) -> CreateButton {
    CreateButton::new(id).emoji(emoji).label(name)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let _test = interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                    .content(format!("Choose a subcommand"))
                    .button(button("changepp",
                        "Update profile picture",
                        "🖼️".parse().unwrap(),
                    ))
                    .button(button("perfs",
                        "Server performances",
                        "<:perfs:1340397611051388950>".parse().unwrap(),
                    ).style(ButtonStyle::Secondary)),
            ),
        )
        .await.unwrap();
        // .unwrap_or_else(|e| {
        //   println!("Error '{e}' while creating response");
        //   () 
        // });
    let m = interaction.get_response(ctx).await.unwrap();

        
    // Wait for multiple interactions
    let mut interaction_stream = m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let cmd = &interaction.data.custom_id;
        // Acknowledge the interaction and send a reply
        if cmd == "perfs" {
            log_system_load(ctx, &interaction).await;
        } else if cmd == "changepp" {
            update_pp(&ctx.http, &mut ctx.http.get_current_user().await.unwrap()).await;
            interaction.defer(ctx).await.unwrap();
        }
    }

    // Delete the orig message or there will be dangling components (components that still
    // exist, but no collector is running so any user who presses them sees an error)
    m.delete(&ctx).await.unwrap();
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("admin").description("Admin panel view")
}
