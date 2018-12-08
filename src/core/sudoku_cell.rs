use super::SudokuValue;

// SudokuCell is a cell in the 9x9 Sudoku grid. It can be Empty (if
// nothing has been written into the cell), or Filled (if we have chosen
// a value here).
#[derive(Clone, Copy, Debug)]
pub enum SudokuCell {
  Empty,
  Filled(SudokuValue),
}
