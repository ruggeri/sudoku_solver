use super::{SudokuPosition, SUDOKU_DIM_SQRT_U8};

// Represents a 3x3 "box" in the Sudoku grid.
#[derive(Clone, Copy)]
pub struct SudokuBox {
  top_left_position: SudokuPosition,
}

impl SudokuBox {
  // `for_position` returns the SudokuBox in which the given
  // SudokuPosition lives.
  pub fn for_position(position: SudokuPosition) -> SudokuBox {
    let (row_idx, col_idx) = position.as_usize_pair();

    // Rounds position down to nearest multiple of 3.
    SudokuBox {
      top_left_position: SudokuPosition::new(
        SUDOKU_DIM_SQRT_U8 * ((row_idx as u8) / SUDOKU_DIM_SQRT_U8),
        SUDOKU_DIM_SQRT_U8 * ((col_idx as u8) / SUDOKU_DIM_SQRT_U8),
      ),
    }
  }

  // `top_left_position` returns the SudokuPosition for the top left
  // corner of the SudokuBox.
  pub fn top_left_position(self) -> SudokuPosition {
    self.top_left_position
  }

  // `positions` returns an iterator over the SudokuPositions in this
  // SudokuBox.
  pub fn positions(self) -> impl Iterator<Item = SudokuPosition> {
    (0..SUDOKU_DIM_SQRT_U8).flat_map(move |rel_row_idx| {
      (0..SUDOKU_DIM_SQRT_U8).map(move |rel_col_idx| {
        self.top_left_position().add(rel_row_idx, rel_col_idx)
      })
    })
  }
}
