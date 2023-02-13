
#[derive(Debug)]
pub enum CreateGameError {
    DimensionTooSmall,
    TooFewPlayers,
    GoalTooSmall,
    GoalTooLarge
}

#[derive(Debug)]
pub enum PlaceMoveError {
    PositionOutOfRange,
    TileNotEmpty
}
