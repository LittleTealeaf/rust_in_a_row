use crate::game_interface::{GameState, GameTrait, PlayMoveError, Tile};

pub enum CreateGameError {
    TooSmallDimensions,
}

pub struct CoreGame {
    width: usize,
    height: usize,
    players: usize,
    turn: usize,
    goal: usize,
    state: Vec<Tile>,
}

impl CoreGame {
    fn new(width: usize, height: usize, goal: usize, players: usize) -> CoreGame {
        let state_length = height * width;
        let mut state = Vec::with_capacity(state_length);
        for _ in 0..state_length {
            state.push(Tile::Empty);
        }

        CoreGame {
            state,
            width,
            height,
            players,
            goal,
            turn: 0,
        }
    }

    pub fn create(
        width: usize,
        height: usize,
        goal: usize,
        players: usize,
    ) -> Result<CoreGame, CreateGameError> {
        if width <= 1 || height <= 1 {
            Err(CreateGameError::TooSmallDimensions)
        } else {
            Ok(CoreGame::new(width, height, goal, players))
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.state[y + self.width * x] = tile;
    }
}

impl GameTrait for CoreGame {
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.state.get(y * self.width + x)
    }

    fn get_game_state(&self) -> GameState {
        let mut move_possible = false;
        let height = self.height as isize;
        let width = self.width as isize;
        let goal = self.goal as isize;

        for x in 0..width {
            for y in 0..height {
                if let Some(Tile::Player(player)) = self.get_tile(x as usize, y as usize) {
                    'directions: for (dx, dy) in [(-1, 0), (-1, 1), (0, -1), (-1, -1)] {
                        let end = x + dx * (goal - 1);
                        if end >= width || end < 0 {
                            continue 'directions;
                        }
                        let end = y + dy * (goal - 1);
                        if end >= height || end < 0 {
                            continue 'directions;
                        }

                        for i in 0..goal {
                            let tx = (x + dx * i) as usize;
                            let ty = (y + dy * i) as usize;

                            if let Some(Tile::Player(p)) = self.get_tile(tx, ty) {
                                if p != player {
                                    continue 'directions;
                                }
                            } else {
                                continue 'directions;
                            }

                            return GameState::PlayerWon(player.clone());
                        }
                    }
                } else {
                    move_possible = true;
                }
            }
        }

        if move_possible {
            GameState::PlayerMove(self.get_current_player())
        } else {
            GameState::Draw
        }
    }

    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError> {
        match self.get_tile(x, y) {
            Some(tile) => match tile {
                Tile::Empty => {
                    self.set_tile(x, y, Tile::Player(self.get_current_player()));
                    self.turn = (self.turn + 1) % self.players;
                    Ok(())
                }
                Tile::Player(_) => Err(PlayMoveError::TileNotEmpty),
            },
            None => Err(PlayMoveError::IndexOutOfBounds),
        }
    }

    fn get_current_player(&self) -> usize {
        self.turn
    }

    fn get_player_count(&self) -> usize {
        self.players
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_goal(&self) -> usize {
        self.goal
    }
}

impl Default for CoreGame {
    fn default() -> Self {
        Self::new(6, 6, 4, 2)
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    #[test]
    fn get_game_state() {
        let mut game = CoreGame::default();

        assert!(
            match game.get_game_state() {
                GameState::PlayerMove(0) => true,
                _ => false,
            },
            "A new game should start with player 0"
        );

        game.play_move(0, 0);

        assert!(
            match game.get_game_state() {
                GameState::PlayerMove(1) => true,
                _ => false,
            },
            "Playing a move should cause the game state to be the next players move"
        );

        game.play_move(0,1);
        game.play_move(1,0);
        game.play_move(1,1);
        game.play_move(2,0);
        game.play_move(2,1);
        game.play_move(3,0);

        assert!(match game.get_game_state() {GameState::PlayerWon(0) => true, _ => false}, "Player 0 should have won");
    }
}
