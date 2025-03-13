use serenity::{
    all::{
        CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateMessage },
    builder::CreateCommand,
};

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {
    // let msg = match interaction.data.resolved.messages.values().next() {
    //     Some(msg) => msg.content.clone(),
    //     None => "".to_owned()
    // };
    let msg = unwrap_or_return!(interaction.data.resolved.messages.values().next());
    // let msg_to_send = format!("{}({} <{}>) : {}", msg.author.display_name(), msg.author.name, msg.author.email.clone().unwrap_or("unavailable email".to_string()), msg.content);
    let msg_to_send = format!("{}({}) : {}", msg.author.display_name(), msg.author.name, msg.content);
    let _ = interaction
        .user
        .direct_message(ctx, CreateMessage::new().content(truncate(&msg_to_send, 2000)))
        .await;
    // let _ = interaction.create_response(
    //     ctx,
    //     serenity::all::CreateInteractionResponse::Message(
    //         CreateInteractionResponseMessage::default().content("Saved !"),
    //     ),
    // );
    let _test = interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default().content(format!("Saved !")),
            ),
        )
        .await
        .unwrap();
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
