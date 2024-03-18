use std::fmt;

pub enum MoveResults {
    MoveMade,
    MoveOutOfBounds,
    SpaceIsAlreadyPlacedByPlayer,
    SpaceIsAlreadyPlacedByOponent,
    NotPlayerTurn,
    GameIsFinished,
    MoveWithInvalidColor,
}

impl fmt::Display for MoveResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoveResults::MoveMade => write!(f, "MoveMade"),
            MoveResults::MoveOutOfBounds => write!(f, "MoveOutOfBounds"),
            MoveResults::SpaceIsAlreadyPlacedByPlayer => write!(f, "SpaceIsAlreadyPlacedByPlayer"),
            MoveResults::SpaceIsAlreadyPlacedByOponent => write!(f, "SpaceIsAlreadyPlacedByOponent"),
            MoveResults::NotPlayerTurn => write!(f, "NotPlayerTurn"),
            MoveResults::GameIsFinished => write!(f, "GameIsFinished"),
            MoveResults::MoveWithInvalidColor => write!(f, "MoveWithInvalidColor"),

            _ => write!(f, "UNKNOWN")
        }
    }
}