pub mod commands;

use serenity::model::application::command::Command;

use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use std::sync::Arc;

use serenity::async_trait;
use serenity::framework::standard::StandardFramework;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use tokio::task::JoinHandle;

use crate::error::DiscordGPTError;

pub struct DiscordService {
    pub bot_token: String,
    pub open_ai_client: Arc<openairs::client::OpenAIClient>,
}

impl DiscordService {
    pub fn new(bot_token: String, open_ai_client: Arc<openairs::client::OpenAIClient>) -> Self {
        DiscordService {
            bot_token,
            open_ai_client,
        }
    }

    pub async fn spawn(self) -> JoinHandle<Result<(), DiscordGPTError>> {
        let framework = StandardFramework::new().configure(|c| c);

        // Configure the client with your Discord bot token in the environment.
        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&self.bot_token, intents)
            .event_handler(Handler {
                open_ai_client: self.open_ai_client.clone(),
            })
            .framework(framework)
            .await
            .expect("Err creating client");

        tokio::spawn(async move { client.start().await.map_err(DiscordGPTError::SerenityError) })
    }
}

struct Handler {
    pub open_ai_client: Arc<openairs::client::OpenAIClient>,
}

impl Handler {
    pub async fn send_defer_message(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> serenity::Result<()> {
        //TODO: Add random waiting message from const waiting messages

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("Hmmm let me think about that for a second")
                    })
            })
            .await
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "davinci" => {
                    //Send defer message to indicate the bot is thinking
                    if let Err(why) = self.send_defer_message(&ctx, &command).await {
                        println!("Something went wrong: {why}");
                    }

                    //Generate the response content
                    let content =
                        commands::davinci::run(&command.data.options, self.open_ai_client.clone())
                            .await;

                    //Send a follow up message with the response content
                    if let Err(why) = command
                        .create_followup_message(&ctx.http, |response| response.content(content))
                        .await
                    {
                        println!("Cannot respond to slash command: {why}");
                    }
                }
                "ping" => {
                    let content = commands::ping::run(&command.data.options);

                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(content))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }

                other => {
                    println!("Slash command unrecognized: {}", other)
                }
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        //Initialize ping command
        Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await
        .expect("Could not create global command ");

        //Initialize davinci command
        Command::create_global_application_command(&ctx.http, |command| {
            commands::davinci::register(command)
        })
        .await
        .expect("Could not create global command ");
    }
}
