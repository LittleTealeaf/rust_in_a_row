use crate::game_interface::{GameState, GameTrait, PlayMoveError, Tile};

pub struct TermGame<T: GameTrait> {
    game: T,
}

impl<T: GameTrait> GameTrait for TermGame<T> {
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.game.get_tile(x, y)
    }

    fn get_game_state(&self) -> GameState {
        self.game.get_game_state()
    }

    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError> {
        self.game.play_move(x, y)
    }

    fn get_current_player(&self) -> usize {
        self.game.get_current_player()
    }

    fn get_player_count(&self) -> usize {
        self.game.get_player_count()
    }

    fn get_width(&self) -> usize {
        self.game.get_width()
    }

    fn get_height(&self) -> usize {
        self.game.get_height()
    }

    fn get_goal(&self) -> usize {
        self.game.get_goal()
    }
}

impl<T: GameTrait> TermGame<T> {}

impl<T: GameTrait> From<T> for TermGame<T> {
    fn from(game: T) -> Self {
        Self { game }
    }
}
