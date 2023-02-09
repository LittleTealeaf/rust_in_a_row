
pub enum CreateGameError {
    InvalidDimension {
        width: usize,
        height: usize
    },
    NotEnoughPlayers {
        players: usize
    },
    GoalTooLarge {
        goal: usize,
        maximum: usize
    },
}
