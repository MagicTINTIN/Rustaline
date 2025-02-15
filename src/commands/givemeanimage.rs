use std::path::*;

use serenity::all::{Context, CreateAttachment, CreateCommand};
use serenity::builder::{
    CreateInteractionResponse, CreateInteractionResponseMessage
};

use serenity::model::prelude::*;
// use tokio::fs::File;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    // let _ = interaction.defer(&ctx.http).await;
    // let mut file = File::open("foo.txt").await.unwrap();
    let path = Path::new("./assets/emojis/rust.png");
    print!("FILE PATH: {}", path.to_string_lossy());
    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::default()
                .add_file(CreateAttachment::path(path).await.unwrap())
                .content(format!("user rust")),
            ),
        )
        .await
        .unwrap();
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("givemeanimage").description("Let me give you a picture")
}
