pub mod errors {
    pub enum NewGameError {
        NotEnoughPlayers,
        ImpossibleWinLength,
        InvalidDimensions,
    }
    pub enum PutPlayerError {
        IndexOutOfBounds,
        PositionNotEmpty,
        PlayerOutOfRange,
    }
}

pub enum Tile {
    Empty,
    Player(usize),
}

pub enum GameState {
    Winner(usize),
    InProgress,
    Draw,
}

pub struct Game {
    board: Vec<Tile>,
    height: usize,
    width: usize,
    win_length: usize,
    players: usize,
}

impl Game {
    pub fn new(
        width: usize,
        height: usize,
        win_length: usize,
        players: usize,
    ) -> Result<Game, errors::NewGameError> {
        if players < 2 {
            return Err(errors::NewGameError::NotEnoughPlayers);
        }

        if width == 0 || height == 0 {
            return Err(errors::NewGameError::InvalidDimensions);
        }

        if win_length > height && win_length > width {
            return Err(errors::NewGameError::ImpossibleWinLength);
        }

        let mut board = Vec::new();

        for _ in 0..(width * height) {
            board.push(Tile::Empty);
        }

        Ok(Game {
            board,
            width,
            height,
            win_length,
            players,
        })
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.board[y * self.width + x])
        }
    }

    pub fn put(&mut self, x: usize, y: usize, player: usize) -> Result<(), errors::PutPlayerError> {
        if player >= self.players {
            return Err(errors::PutPlayerError::PlayerOutOfRange);
        }

        match self.get(x, y) {
            Some(tile) => match tile {
                Tile::Empty => {
                    self.board[y * self.width + x] = Tile::Player(player);
                    Ok(())
                }
                Tile::Player(_) => Err(errors::PutPlayerError::PositionNotEmpty),
            },
            None => Err(errors::PutPlayerError::IndexOutOfBounds),
        }
    }

    pub fn winner(&self) -> GameState {
        let mut move_possible = false;
        let height = self.height as isize;
        let width = self.width as isize;
        let win_length = self.win_length as isize;
        for x in 0..width {
            for y in 0..height {
                if let Tile::Player(player) = &self.board[(y * width + x) as usize] {
                    'directions: for (dx, dy) in [(-1, 0), (-1, 1), (0, -1), (-1, -1)] {
                        let end = x + dx * (win_length - 1);
                        if end >= width || end < 0 {
                            continue 'directions;
                        }
                        let end = y + dy * (win_length - 1);
                        if end >= height || end < 0 {
                            continue 'directions;
                        }

                        for i in 0..win_length {
                            let tx = (x + dx * i) as usize;
                            let ty = (y + dy * i) as usize;

                            if let Tile::Player(p) = &self.board[ty * self.width + tx] {
                                if p != player {
                                    continue 'directions;
                                }
                            } else {
                                continue 'directions;
                            }
                        }

                        return GameState::Winner(player.clone());
                    }
                } else {
                    move_possible = true;
                }
            }
        }

        if move_possible {
            GameState::InProgress
        } else {
            GameState::Draw
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        assert!(
            match Game::new(10, 10, 5, 2) {
                Ok(_) => true,
                _ => false,
            },
            "Valid game configurations should return an OK value"
        );
        assert!(
            match Game::new(10, 10, 5, 0) {
                Err(errors::NewGameError::NotEnoughPlayers) => true,
                _ => false,
            },
            "A new game cannot have 0 players"
        );
        assert!(
            match Game::new(10, 10, 5, 1) {
                Err(errors::NewGameError::NotEnoughPlayers) => true,
                _ => false,
            },
            "A new game cannot have 1 player"
        );
        assert!(
            match Game::new(0, 10, 5, 3) {
                Err(errors::NewGameError::InvalidDimensions) => true,
                _ => false,
            },
            "A new game cannot have 0 width"
        );

        assert!(
            match Game::new(10, 0, 5, 3) {
                Err(errors::NewGameError::InvalidDimensions) => true,
                _ => false,
            },
            "A new game cannot have 0 height"
        );
        assert!(
            match Game::new(5, 5, 10, 2) {
                Err(errors::NewGameError::ImpossibleWinLength) => true,
                _ => false,
            },
            "A new game cannot have a win length greater than any dimension"
        );

        assert!(
            match Game::new(5, 12, 10, 2) {
                Err(errors::NewGameError::ImpossibleWinLength) => false,
                _ => true,
            },
            "A new game should be allowed with height > win length > width"
        );

        assert!(
            match Game::new(12, 5, 10, 2) {
                Err(errors::NewGameError::ImpossibleWinLength) => false,
                _ => true,
            },
            "A new game should be allowed with width > win length > height"
        );
    }
}
