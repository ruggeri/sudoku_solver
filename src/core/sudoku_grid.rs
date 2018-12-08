use super::{SudokuCell, SudokuChoice};
use std::fmt;

// Represents the 9x9 Sudoku grid.
pub struct SudokuGrid {
  // 9x9 cells, each of which can be empty, or filled with a
  // SudokuValue.
  cells: [[SudokuCell; 9]; 9],
}

impl SudokuGrid {
  pub fn default() -> SudokuGrid {
    let cells = [[SudokuCell::Empty; 9]; 9];
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
