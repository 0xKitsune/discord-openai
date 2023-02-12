use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub const PING_RESPONSE: &str = "Pong";

pub fn run(_options: &[CommandDataOption]) -> String {
    PING_RESPONSE.to_owned()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
