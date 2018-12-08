use super::SudokuPosition;
use super::SudokuValue;

// A SudokuChoice is a chosen value for a chosen position. When we make
// a "choice," we are penciling in the given value at the specified
// position.
#[derive(Clone, Copy, Debug)]
pub struct SudokuChoice {
  pub position: SudokuPosition,
  pub value: SudokuValue,
}

impl SudokuChoice {
  pub fn new(
    position: SudokuPosition,
    value: SudokuValue,
  ) -> SudokuChoice {
    SudokuChoice { position, value }
  }
}
