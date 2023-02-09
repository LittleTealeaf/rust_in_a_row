use std::cmp::min;

use super::errors::CreateGameError;

pub enum Tile {
    Empty,
    Player(usize),
}

pub struct Game {
    width: usize,
    height: usize,
    players: usize,
    goal: usize,
    state: Vec<Tile>,
}

impl Game {
    fn new(width: usize, height: usize, goal: usize, players: usize) -> Game {
        let mut state = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            state.push(Tile::Empty);
        }

        Game {
            state,
            width,
            height,
            players,
            goal,
        }
    }

    pub fn create(
        width: usize,
        height: usize,
        goal: usize,
        players: usize,
    ) -> Result<Game, CreateGameError> {
        if width == 0 || height == 0 {
            Err(CreateGameError::InvalidDimension { width, height })
        } else if players < 2 {
            Err(CreateGameError::NotEnoughPlayers { players })
        } else if goal > width || goal > height {
            Err(CreateGameError::GoalTooLarge {
                goal,
                maximum: min(width, height),
            })
        } else {
            Ok(Self::new(width, height, goal, players))
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(6, 6, 4, 2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
