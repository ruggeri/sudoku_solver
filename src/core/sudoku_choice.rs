use super::SudokuValue;
use super::SudokuPosition;

#[derive(Clone, Copy, Debug)]
pub struct SudokuChoice {
  pub position: SudokuPosition,
  pub value: SudokuValue,
}

impl SudokuChoice {
  pub fn new(position: SudokuPosition, value: SudokuValue) -> SudokuChoice {
    SudokuChoice{position, value}
  }
}
