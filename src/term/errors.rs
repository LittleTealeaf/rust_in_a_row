use crate::game::errors::CreateGameError;
use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum PromptNewGameError {
   CreateGame(CreateGameError),
   IO(io::Error),
   ParseError(ParseIntError)
}
