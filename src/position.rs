#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SudokuPosition {
  pub row_idx: u8,
  pub col_idx: u8
}

impl SudokuPosition {
  pub fn all() -> Vec<SudokuPosition> {
    let mut all = Vec::new();

    for row_idx in 0..9 {
      for col_idx in 0..9 {
        all.push(SudokuPosition{row_idx, col_idx});
      }
    }

    all
  }

  pub fn as_usize_pair(self) -> (usize, usize) {
    (self.row_idx as usize, self.col_idx as usize)
  }
}
