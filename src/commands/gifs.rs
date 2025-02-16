use std::time::Duration;

use serenity::all::{Context, CreateButton, CreateCommand};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

use serenity::futures::StreamExt;
use serenity::model::prelude::*;
// use tokio::fs::File;

fn button(id: &str, name: &str, emoji: ReactionType) -> CreateButton {
    CreateButton::new(id).emoji(emoji).label(name)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let _test = interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                    .content(format!("Manage gif commands"))
                    .button(button("add",
                        "Add a gif",
                        "🆕".parse().unwrap(),
                    ))
                    .button(button("remove",
                        "Remove a gif",
                        "❌".parse().unwrap(),
                    ).style(ButtonStyle::Secondary)),
            ),
        )
        .await.unwrap();
    let m = interaction.get_response(ctx).await.unwrap();

        
    // Wait for multiple interactions
    let mut interaction_stream = m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let cmd = &interaction.data.custom_id;
        // Acknowledge the interaction and send a reply
        if cmd == "add" {
            //
        } else if cmd == "remove" {
            //
        }
    }
    m.delete(&ctx).await.unwrap();
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gifs").description("Manage GIFs commands")
}
