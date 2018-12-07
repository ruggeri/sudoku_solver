use super::value::SudokuValue;

#[derive(Clone, Copy)]
pub enum SudokuCell {
  Empty,
  Filled(SudokuValue)
}
