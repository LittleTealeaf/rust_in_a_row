#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    A,
    B,
}

pub enum GameState {
    Decisive(Player),
    Draw,
    InProgress,
}

pub enum Tile {
    Empty,
    Player(Player),
}

pub struct Game {
    board: Vec<Tile>,
    height: usize,
    width: usize,
    win_length: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, win_length: usize) -> Game {
        let mut board = Vec::new();

        for _ in 0..(height * width) {
            board.push(Tile::Empty);
        }

        Game {
            width,
            height,
            win_length,
            board,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.board[y * self.width + x])
        }
    }

    pub fn set(&mut self, x: usize, y: usize, player: Player) -> bool {
        if let Some(tile) = self.get(x, y) {
            if let Tile::Empty = tile {
                self.board[y * self.width + x] = Tile::Player(player);
                return true;
            }
        }
        false
    }

    pub fn winner(&self) -> GameState {
        let mut has_space = false;
        for x in 0..(self.width as isize) {
            for y in 0..(self.height as isize) {
                if let Tile::Player(player) = &self.board[(y * self.width as isize + x) as usize] {
                    'directions: for (dx, dy) in [(-1, 0), (-1, 1), (0, -1), (-1, -1)] {
                        let end = x + dx * self.win_length as isize;
                        if end >= self.width as isize || end < 0 {
                            continue 'directions;
                        }
                        let end = y + dy * self.win_length as isize;
                        if end >= self.height as isize || end < 0 {
                            continue 'directions;
                        }

                        'step: for i in 0..(self.win_length as isize) {
                            let tx = (x + dx * i) as usize;
                            let ty = (y + dy * i) as usize;

                            if let Tile::Player(p) = &self.board[ty * self.width + tx] {
                                if p.eq(player) {
                                    continue 'step;
                                }
                            }

                            if i == self.win_length as isize - 1 {
                                return GameState::Decisive(player.clone());
                            }

                            continue 'directions;
                        }
                    }
                } else {
                    has_space = true;
                }
            }
        }

        if has_space {
            GameState::InProgress
        } else {
            GameState::Draw
        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;




}
