use crate::base::GameTrait;

use super::{CpuTrait, PlayerType};

pub struct CpuGame<T: GameTrait> {
    game: T,
    cpu_players: Vec<usize>,
}

impl<T: GameTrait> CpuGame<T> {}

impl<T: GameTrait> CpuTrait for CpuGame<T> {
    fn get_player_type(&self, player: usize) -> Option<PlayerType> {
        if player >= self.game.get_player_count() {
            None
        } else if self.cpu_players.contains(&player) {
            Some(PlayerType::CPU)
        } else {
            Some(PlayerType::User)
        }
    }

    fn get_players_of_type(&self, player_type: PlayerType) -> Vec<usize> {
    }
}
