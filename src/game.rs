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
    pub fn new(width: usize, height: usize, win_length: usize, players: usize) -> Game {
        let mut board = Vec::new();

        for _ in 0..(width * height) {
            board.push(Tile::Empty);
        }

        Game {
            board,
            width,
            height,
            win_length,
            players,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.board[y * self.width + x])
        }
    }

    pub fn put(&mut self, x: usize, y: usize, player: usize) -> bool {
        assert!(player < self.players);
        if let Some(tile) = self.get(x, y) {
            if let Tile::Empty = tile {
                self.board[y * self.width + x] = Tile::Player(player);
                return true;
            }
        }
        false
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
    fn new_game() {
        let game = Game::new(6,6,4,2);
    }
}
