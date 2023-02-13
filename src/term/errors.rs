use crate::game::errors::CreateGameError;
use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum PromptNewGameError {
    CreateGame(CreateGameError),
    IO(io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for PromptNewGameError {
    fn from(error: io::Error) -> Self {
        PromptNewGameError::IO(error)
    }
}

impl From<ParseIntError> for PromptNewGameError {
    fn from(error: ParseIntError) -> Self {
        PromptNewGameError::ParseError(error)
    }
}

impl From<CreateGameError> for PromptNewGameError {
    fn from(error: CreateGameError) -> Self {
        PromptNewGameError::CreateGame(error)
    }
}
