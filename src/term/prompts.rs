use std::io::{self, stdin};

use crate::game::Game;

use super::errors::PromptNewGameError;

pub fn read_input() -> Result<String, io::Error> {
    let mut line = String::new();
    let result = stdin().read_line(&mut line);
    match result {
        Ok(_) => Ok(line),
        Err(error) => Err(error),
    }
}

pub fn prompt_new_game() -> Result<Game, PromptNewGameError> {
    let width: usize;
    let height: usize;
    let goal: usize;
    let players: usize;

    println!("Board Width");
    match read_input() {
        Ok(line) => match line.trim().parse() {
            Ok(value) => width = value,
            Err(error) => return Err(PromptNewGameError::ParseError(error)),
        },
        Err(error) => return Err(PromptNewGameError::IO(error)),
    }

    println!("Board Height");
    match read_input() {
        Ok(line) => match line.trim().parse() {
            Ok(value) => height = value,
            Err(error) => return Err(PromptNewGameError::ParseError(error)),
        },
        Err(error) => return Err(PromptNewGameError::IO(error)),
    }

    println!("Goal Length");
    match read_input() {
        Ok(line) => match line.trim().parse() {
            Ok(value) => goal = value,
            Err(error) => return Err(PromptNewGameError::ParseError(error)),
        },
        Err(error) => return Err(PromptNewGameError::IO(error)),
    }

    println!("Player Count");
    match read_input() {
        Ok(line) => match line.trim().parse() {
            Ok(value) => players = value,
            Err(error) => return Err(PromptNewGameError::ParseError(error)),
        },
        Err(error) => return Err(PromptNewGameError::IO(error)),
    }

    match Game::create(width, height, goal, players) {
        Ok(game) => Ok(game),
        Err(error) => Err(PromptNewGameError::CreateGame(error)),
    }
}
