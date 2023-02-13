use crate::{
    core_game::{CoreGame, CreateGameError},
    game_interface::{GameState, GameTrait, PlayMoveError, Tile},
};

pub enum NewCpuGameError {
    CpuPlayerOutOfRange,
}

pub enum CreateCpuGameError {
    NewCpuGame(NewCpuGameError),
    CreateGame(CreateGameError),
}

impl From<CreateGameError> for CreateCpuGameError {
    fn from(error: CreateGameError) -> Self {
        Self::CreateGame(error)
    }
}

impl From<NewCpuGameError> for CreateCpuGameError {
    fn from(value: NewCpuGameError) -> Self {
        Self::NewCpuGame(value)
    }
}

pub struct CpuGame<T: GameTrait> {
    core_game: T,
    cpu_players: Vec<usize>,
}

impl<T: GameTrait> CpuGame<T> {
    fn new(game: T, cpu_players: Vec<usize>) -> Result<CpuGame<T>, NewCpuGameError> {
        if cpu_players.iter().max().unwrap() >= &game.get_player_count() {
            Err(NewCpuGameError::CpuPlayerOutOfRange)
        } else {
            Ok(CpuGame {
                core_game: game,
                cpu_players,
            })
        }
    }

    pub fn create(
        width: usize,
        height: usize,
        goal: usize,
        player_count: usize,
        cpu_players: Vec<usize>,
    ) -> Result<CpuGame<CoreGame>, CreateCpuGameError> {
        Ok(CpuGame::new(
            CoreGame::create(width, height, goal, player_count)?,
            cpu_players,
        )?)
    }

    fn get_best_move(&self) -> (usize, usize) {
        (0, 0)
    }
}

impl<T: GameTrait> GameTrait for CpuGame<T> {
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.core_game.get_tile(x, y)
    }

    fn get_game_state(&self) -> GameState {
        self.core_game.get_game_state()
    }

    fn play_move(&mut self, x: usize, y: usize) -> Result<(), PlayMoveError> {
        self.core_game.play_move(x, y)?;

        while {
            match self.get_game_state() {
                GameState::PlayerMove(player) => self.cpu_players.contains(&player),
                _ => false,
            }
        } {
            let (x, y) = self.get_best_move();
            self.play_move(x, y)?;
        }

        Ok(())
    }

    fn get_current_player(&self) -> usize {
        self.core_game.get_current_player()
    }

    fn get_player_count(&self) -> usize {
        self.core_game.get_player_count()
    }

    fn get_width(&self) -> usize {
        self.core_game.get_width()
    }

    fn get_height(&self) -> usize {
        self.core_game.get_height()
    }

    fn get_goal(&self) -> usize {
        self.core_game.get_goal()
    }
}
