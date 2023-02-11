
pub enum CreateGameError {
    DimensionTooSmall,
    TooFewPlayers,
    GoalTooSmall,
    GoalTooLarge
}

pub enum PlaceMoveError {
    PositionOutOfRange,
    TileNotEmpty
}
