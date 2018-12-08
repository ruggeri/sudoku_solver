use std::fmt;
use super::{SudokuCell, SudokuChoice};

pub struct SudokuGrid {
  cells: [[SudokuCell; 9]; 9],
}

impl SudokuGrid {
  pub fn default() -> SudokuGrid {
    let cells = [[SudokuCell::Empty; 9]; 9];
    SudokuGrid { cells }
  }

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
          SudokuCell::Filled(value) => write!(f, "{}", value.as_u8_val())?,
        }
      }

      writeln!(f)?;
    }

    Ok(())
  }
}
