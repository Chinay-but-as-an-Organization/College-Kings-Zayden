use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::respond_with_message;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    respond_with_message(ctx, interaction, "Pong!").await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
