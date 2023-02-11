use std::io;

use crate::game::errors::CreateGameError;

pub enum NewGameError {
    InvalidConfigError(CreateGameError),
    IOError(io::Error),
    RequestCancelled,
}
