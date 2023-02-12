use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use std::sync::Arc;

pub const DAVINCI_ERROR_MSG: &str = "Something went wrong when talking to DAVINCI";
pub const NOT_SURE_HOW_YOU_GOT_HERE_ERROR: &str = "I'm not sure how you managed to send this request without a prompt. Not even an empty string. Congratulations, I think?";
pub const RESOLUTION_ERROR: &str =
    "Something went wrong while trying to process your discord message";
pub const NO_PROMPT_PROVIDED: &str = "Please supply a prompt with your message";

pub async fn run(
    options: &[CommandDataOption],
    open_ai_client: Arc<openairs::client::OpenAIClient>,
) -> String {
    if let Some(command_data_option) = options.get(0) {
        if let Some(command_data_option_value) = &command_data_option.resolved {
            if let CommandDataOptionValue::String(prompt) = command_data_option_value {
                if prompt.is_empty() {
                    return NO_PROMPT_PROVIDED.to_owned();
                }

                match open_ai_client
                    .complete(&openairs::models::TEXT_DAVINCI_003, prompt)
                    .await
                {
                    Ok(response) => response.choices[0].text.to_owned(),
                    Err(_) => DAVINCI_ERROR_MSG.to_owned(),
                }
            } else {
                NOT_SURE_HOW_YOU_GOT_HERE_ERROR.to_owned()
            }
        } else {
            RESOLUTION_ERROR.to_owned()
        }
    } else {
        NO_PROMPT_PROVIDED.to_owned()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("davinci")
        .description("Ask Davinci anything")
        .create_option(|option| {
            option
                .name("prompt")
                .description("Supply any prompt to Davinci")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
