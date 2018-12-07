use super::value::SudokuValue;

#[derive(Clone, Copy, Debug)]
pub enum SudokuCell {
  Empty,
  Filled(SudokuValue)
}
