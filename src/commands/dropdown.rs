use std::time::Duration;

use serenity::all::{Context, CreateCommand};
use serenity::builder::{
    CreateButton,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    CreateMessage,
    CreateSelectMenu,
    CreateSelectMenuKind,
    CreateSelectMenuOption,
};

use serenity::futures::StreamExt;
use serenity::model::prelude::*;

fn button(name: &str, emoji: ReactionType) -> CreateButton {
    // To add an emoji to buttons, use .emoji(). The method accepts anything ReactionType or
    // anything that can be converted to it. For a list of that, search Trait Implementations in
    // the docs for From<...>.
    CreateButton::new(name).emoji(emoji)
}

pub async fn send_drop_down(ctx: &Context, channel_id: &ChannelId) {
    // Ask the user for its favorite animal
    let message = channel_id
        .send_message(
            &ctx,
            CreateMessage::new()
                .content("Please select your favorite animal")
                .select_menu(
                    CreateSelectMenu::new(
                        "animal_select",
                        CreateSelectMenuKind::String {
                            options: vec![
                                CreateSelectMenuOption::new("🐈 meow", "Cat"),
                                CreateSelectMenuOption::new("🐕 woof", "Dog"),
                                CreateSelectMenuOption::new("🦀 use rust", "Crab"),
                            ],
                        },
                    )
                    .custom_id("animal_select")
                    .placeholder("No animal selected"),
                ),
        )
        .await;
    let m = message
        .unwrap();

    // Wait for the user to make a selection
    // This uses a collector to wait for an incoming event without needing to listen for it
    // manually in the EventHandler.
    let interaction = match m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        Some(x) => x,
        None => {
            m.reply(&ctx, "Timed out").await.unwrap();
            return;
        }
    };

    // data.values contains the selected value from each select menus. We only have one menu,
    // so we retrieve the first
    let animal = match &interaction.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => &values[0],
        _ => panic!("unexpected interaction data kind"),
    };

    // Acknowledge the interaction and edit the message
    interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::default()
                    .content(format!("You chose: **{animal}**\nNow choose a button!"))
                    .button(button("meow", "🐈".parse().unwrap()))
                    .button(button("woof", "🐕".parse().unwrap()))
                    .button(button(
                        "use rust",
                        // Custom emojis in Discord are represented with
                        // `<:EMOJI_NAME:EMOJI_ID>`. You can see this by posting an emoji in
                        // your server and putting a backslash before the emoji.
                        //
                        // Because ReactionType implements FromStr, we can use .parse() to
                        // convert the textual emoji representation to ReactionType
                        "<:rust:1340189238162886806>".parse().unwrap(),
                    )),
            ),
        )
        .await
        .unwrap();

    // Wait for multiple interactions
    let mut interaction_stream = m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let sound = &interaction.data.custom_id;
        // Acknowledge the interaction and send a reply
        interaction
            .create_response(
                &ctx,
                // This time we dont edit the message but reply to it
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::default()
                        // Make the message hidden for other users by setting `ephemeral(true)`.
                        .ephemeral(true)
                        .content(format!("The **{animal}** says __{sound}__")),
                ),
            )
            .await
            .unwrap();
    }

    // Delete the orig message or there will be dangling components (components that still
    // exist, but no collector is running so any user who presses them sees an error)
    m.delete(&ctx).await.unwrap()
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // let _ = interaction.defer(&ctx.http).await;
    interaction.create_response(ctx, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::default()
                        // Make the message hidden for other users by setting `ephemeral(true)`.
                        .ephemeral(true)
                        .content(format!("veri efemeral")),
    )).await.unwrap();
    interaction.delete_response(ctx).await.unwrap();
    send_drop_down(ctx, &interaction.channel_id).await;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("quiz").description("Do you really know animals ?")
}
