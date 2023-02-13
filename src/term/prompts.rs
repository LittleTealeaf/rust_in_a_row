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
    println!("Board Width");
    let width = read_input()?.trim().parse()?;

    println!("Board Height");
    let height = read_input()?.trim().parse()?;

    println!("Goal Length");
    let goal = read_input()?.trim().parse()?;

    println!("Player Count");
    let players = read_input()?.trim().parse()?;

    Ok(Game::create(width, height, goal, players)?)
}


