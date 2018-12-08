use super::{SudokuCell, SudokuChoice, SUDOKU_DIM_USIZE};
use std::fmt;

// Represents the 9x9 Sudoku grid.
pub struct SudokuGrid {
  // 9x9 cells, each of which can be empty, or filled with a
  // SudokuValue.
  cells: [[SudokuCell; SUDOKU_DIM_USIZE]; SUDOKU_DIM_USIZE],
}

impl SudokuGrid {
  pub fn default() -> SudokuGrid {
    let cells =
      [[SudokuCell::Empty; SUDOKU_DIM_USIZE]; SUDOKU_DIM_USIZE];
    SudokuGrid { cells }
  }

  // `place` plays a SudokuChoice: it writes the given SudokuValue in at
  // the given SudokuPosition in the grid.
  //
  // We allow the caller to make invalid choices, and to overwrite
  // previously filled values.
  pub fn place(&mut self, choice: SudokuChoice) {
    let (row_idx, col_idx) = choice.position.as_usize_pair();
    self.cells[row_idx][col_idx] = SudokuCell::Filled(choice.value);
  }
}

impl fmt::Display for SudokuGrid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in &self.cells {
      for cell in row {
        match cell {
          SudokuCell::Empty => write!(f, ".")?,
          SudokuCell::Filled(value) => {
            write!(f, "{}", value.as_u8_value())?
          }
        }
      }

      writeln!(f)?;
    }

    Ok(())
  }
}
