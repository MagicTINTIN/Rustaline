use serenity::{
    all::{CommandInteraction, Context, CreateMessage},
    builder::CreateCommand,
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    let _ = interaction.user.direct_message(
        ctx,
        CreateMessage::new().content("Blabla"),
    ).await;
    // interaction.defer_ephemeral(ctx);
}

pub fn register() -> CreateCommand {
    CreateCommand::new("save")
        // .description("Save a message in DM")
        .kind(serenity::all::CommandType::Message)
        .contexts(vec![
            serenity::all::InteractionContext::Guild,
            serenity::all::InteractionContext::BotDm,
            serenity::all::InteractionContext::PrivateChannel,
        ])
    // InteractionContext::BotDm
}
