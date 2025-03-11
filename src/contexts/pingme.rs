use serenity::all::InteractionContext;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "PING!".to_string()
}

pub fn register() -> InteractionContext {
    // CreateCommand::new("pingme").description("A ping command")
    InteractionContext::BotDm
}