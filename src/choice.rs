use super::value::SudokuValue;
use super::position::SudokuPosition;

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
