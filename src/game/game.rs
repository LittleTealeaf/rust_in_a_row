use super::errors::{CreateGameError, PlaceMoveError};

pub enum Tile {
    Empty,
    Player(usize),
}

pub struct Game {
    width: usize,
    height: usize,
    players: usize,
    turn: usize,
    goal: usize,
    state: Vec<Tile>,
}

pub trait GameTrait {
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile>;
    fn get_turn(&self) -> usize;
    fn place_move(&mut self, x: usize, y: usize) -> Result<(), PlaceMoveError>;
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
            turn: 0,
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.state[y * self.width + x] = tile
    }

    fn next_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players;
    }

    pub fn create(
        width: usize,
        height: usize,
        goal: usize,
        players: usize,
    ) -> Result<Game, CreateGameError> {
        if width < 3 || height < 3 {
            Err(CreateGameError::DimensionTooSmall)
        } else if players < 2 {
            Err(CreateGameError::TooFewPlayers)
        } else if goal < 3 {
            Err(CreateGameError::GoalTooSmall)
        } else if goal > width || goal > height {
            Err(CreateGameError::GoalTooLarge)
        } else {
            Ok(Self::new(width, height, goal, players))
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.state.get(y * self.width + x)
    }

    pub fn get_turn(&self) -> usize {
        self.turn
    }

    pub fn place_move(&mut self, x: usize, y: usize) -> Result<(), PlaceMoveError> {
        match self.get_tile(x, y) {
            Some(tile) => match tile {
                Tile::Empty => {
                    self.set_tile(x, y, Tile::Player(self.get_turn()));
                    self.next_turn();
                    Ok(())
                }
                Tile::Player(_) => Err(PlaceMoveError::TileNotEmpty),
            },
            None => Err(PlaceMoveError::PositionOutOfRange),
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

    #[test]
    fn create() {
        assert!(match Game::create(6, 6, 4, 2) {
            Ok(_) => true,
            _ => false,
        });

        assert!(match Game::create(2, 6, 3, 2) {
            Err(CreateGameError::DimensionTooSmall) => true,
            _ => false,
        });

        assert!(match Game::create(6, 2, 3, 2) {
            Err(CreateGameError::DimensionTooSmall) => true,
            _ => false,
        });

        assert!(match Game::create(6, 6, 2, 2) {
            Err(CreateGameError::GoalTooSmall) => true,
            _ => false,
        });

        assert!(match Game::create(6, 6, 7, 2) {
            Err(CreateGameError::GoalTooLarge) => true,
            _ => false,
        });

        assert!(match Game::create(6, 6, 4, 0) {
            Err(CreateGameError::TooFewPlayers) => true,
            _ => false,
        });
    }

    #[test]
    fn default() {
        let game = Game::default();
        assert_eq!(game.width, 6);
        assert_eq!(game.height, 6);
        assert_eq!(game.players, 2);
        assert_eq!(game.goal, 4);
    }

    #[test]
    fn next_turn() {
        {
            let mut game = Game::default();
            assert_eq!(game.get_turn(), 0);
            game.next_turn();
            assert_eq!(game.get_turn(), 1);
            game.next_turn();
            assert_eq!(game.get_turn(), 0);
        }

        {
            let mut game = Game::new(6, 6, 4, 4);
            assert_eq!(game.get_turn(), 0);
            game.next_turn();
            assert_eq!(game.get_turn(), 1);
            game.next_turn();
            assert_eq!(game.get_turn(), 2);
            game.next_turn();
            assert_eq!(game.get_turn(), 3);
            game.next_turn();
            assert_eq!(game.get_turn(), 0);
        }
    }
}
