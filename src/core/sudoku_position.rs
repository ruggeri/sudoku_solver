#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SudokuPosition {
  row_idx: u8,
  col_idx: u8
}

impl SudokuPosition {
  pub fn new(row_idx: u8, col_idx: u8) -> SudokuPosition {
    // Note that I don't check 0 <= ... because u8 is by definition
    // unsigned.
    assert!(row_idx < 9 && col_idx < 9);

    SudokuPosition { row_idx, col_idx }
  }

  pub fn all() -> Vec<SudokuPosition> {
    let mut all = Vec::new();

    for row_idx in 0..9 {
      for col_idx in 0..9 {
        all.push(SudokuPosition{row_idx, col_idx});
      }
    }

    all
  }

  pub fn add(self, row_offset: u8, col_offset: u8) -> SudokuPosition {
    SudokuPosition::new(
      self.row_idx + row_offset,
      self.col_idx + col_offset
    )
  }

  pub fn as_usize_pair(self) -> (usize, usize) {
    (self.row_idx as usize, self.col_idx as usize)
  }
}
