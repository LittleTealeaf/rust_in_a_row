pub enum Tile {
    Empty,
    Player(usize),
}

pub enum PlayMoveError {
    IndexOutOfBounds,
    TileNotEmpty,
}

pub enum GameState {
    PlayerMove(usize),
    PlayerWon(usize),
    Draw,
}


pub trait GameTrait {
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile>;
    fn get_game_state(&self) -> GameState;
    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError>;
    fn get_current_player(&self) -> usize;
    fn get_player_count(&self) -> usize;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_goal(&self) -> usize;
}


