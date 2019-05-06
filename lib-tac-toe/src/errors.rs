use failure::{Error, Fail};

use super::Outcome;

#[derive(Debug, Fail)]
pub enum MarkError {
    #[fail(display = "Cell index {} out of bounds; max index is {}", index, max)]
    OutOfBounds { index: usize, max: usize },

    #[fail(display = "Cell is already marked!")]
    CellMarked,

    #[fail(display = "Game already finished!")]
    GameEnded,
}

impl MarkError {
    pub(super) fn new_oob(index: usize) -> Self {
        MarkError::OutOfBounds {
            index,
            max: super::MAX_INDEX,
        }
    }
}

pub type MarkResult = Result<Option<Outcome>, Error>;
