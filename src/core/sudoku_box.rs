use super::SudokuPosition;

// Represents a 3x3 "box" in the Sudoku grid.
#[derive(Clone, Copy)]
pub enum SudokuBox {
  TopLeft,
  TopCenter,
  TopRight,
  CenterLeft,
  CenterCenter,
  CenterRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

impl SudokuBox {
  // `for_position` returns the SudokuBox in which the given
  // SudokuPosition lives.
  pub fn for_position(position: SudokuPosition) -> SudokuBox {
    use self::SudokuBox::*;

    let (row_idx, col_idx) = position.as_usize_pair();

    match (row_idx / 3, col_idx / 3) {
      (0, 0) => TopLeft,
      (0, 1) => TopCenter,
      (0, 2) => TopRight,
      (1, 0) => CenterLeft,
      (1, 1) => CenterCenter,
      (1, 2) => CenterRight,
      (2, 0) => BottomLeft,
      (2, 1) => BottomCenter,
      (2, 2) => BottomRight,
      _ => unreachable!("boxes should have proper index"),
    }
  }

  // `top_left_position` returns the SudokuPosition for the top left
  // corner of the SudokuBox.
  pub fn top_left_position(self) -> SudokuPosition {
    use self::SudokuBox::*;

    let (row_idx, col_idx) = match self {
      TopLeft => (0, 0),
      TopCenter => (0, 3),
      TopRight => (0, 6),
      CenterLeft => (3, 0),
      CenterCenter => (3, 3),
      CenterRight => (3, 6),
      BottomLeft => (6, 0),
      BottomCenter => (6, 3),
      BottomRight => (6, 6),
    };

    SudokuPosition::new(row_idx, col_idx)
  }

  // `positions` returns an iterator over the SudokuPositions in this
  // SudokuBox.
  pub fn positions(self) -> impl Iterator<Item = SudokuPosition> {
    (0..3).flat_map(move |rel_row_idx| {
      (0..3).map(move |rel_col_idx| {
        self.top_left_position().add(rel_row_idx, rel_col_idx)
      })
    })
  }
}
