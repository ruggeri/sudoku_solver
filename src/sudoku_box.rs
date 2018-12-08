use super::position::SudokuPosition;

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
      _ => {
        unreachable!("boxes should have proper index")
      }
    }
  }

  pub fn top_left_position(self) -> SudokuPosition {
    use self::SudokuBox::*;

    match self {
      TopLeft => SudokuPosition::new(0, 0),
      TopCenter => SudokuPosition::new(0, 3),
      TopRight => SudokuPosition::new(0, 6),
      CenterLeft => SudokuPosition::new(3, 0),
      CenterCenter => SudokuPosition::new(3, 3),
      CenterRight => SudokuPosition::new(3, 6),
      BottomLeft => SudokuPosition::new(6, 0),
      BottomCenter => SudokuPosition::new(6, 3),
      BottomRight => SudokuPosition::new(6, 6),
    }
  }

  pub fn to_usize_idx(self) -> usize {
    use self::SudokuBox::*;

    match self {
      TopLeft => 0,
      TopCenter => 1,
      TopRight => 2,
      CenterLeft => 3,
      CenterCenter => 4,
      CenterRight => 5,
      BottomLeft => 6,
      BottomCenter => 7,
      BottomRight => 8,
    }
  }

  pub fn positions(self) -> impl Iterator<Item=SudokuPosition> {
    (0..3).flat_map(move |rel_row_idx| {
      (0..3).map(move |rel_col_idx| {
        self.top_left_position().add(rel_row_idx, rel_col_idx)
      })
    })
  }
}
