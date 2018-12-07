use super::position::SudokuPosition;

#[derive(Clone, Copy)]
pub struct BoxCoord(pub u8, pub u8);

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

  pub fn to_box_coord(self) -> BoxCoord {
    use self::SudokuBox::*;

    match self {
      TopLeft => BoxCoord(0, 0),
      TopCenter => BoxCoord(0, 1),
      TopRight => BoxCoord(0, 2),
      CenterLeft => BoxCoord(1, 0),
      CenterCenter => BoxCoord(1, 1),
      CenterRight => BoxCoord(1, 2),
      BottomLeft => BoxCoord(2, 0),
      BottomCenter => BoxCoord(2, 1),
      BottomRight => BoxCoord(2,2),
    }
  }

  pub fn to_usize(self) -> usize {
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
}
