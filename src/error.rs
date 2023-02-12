use serenity::prelude::SerenityError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum DiscordGPTError {
    #[error("Serenity error")]
    SerenityError(#[from] SerenityError),
    #[error("Tokio Join error")]
    JoinError(#[from] JoinError),
}
