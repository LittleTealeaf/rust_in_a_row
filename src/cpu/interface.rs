pub enum PlayerType {
    CPU,
    User,
}

pub trait CpuTrait {
    fn get_player_type(&self, player: usize) -> Option<PlayerType>;
    fn get_players_of_type(&self, player_type: PlayerType) -> Vec<usize>;
}
