#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SudokuValue(u8);

impl SudokuValue {
  pub fn first() -> SudokuValue {
    SudokuValue(1)
  }

  pub fn new(val: u8) -> SudokuValue {
    assert!(1 <= val && val <= 9);
    SudokuValue(val)
  }

  pub fn as_u8_val(self) -> u8 {
    let SudokuValue(val) = self;
    val
  }

  pub fn as_idx(self) -> usize {
    (self.as_u8_val() as usize) - 1
  }

  pub fn next(self) -> Option<SudokuValue> {
    let val = self.as_u8_val();

    if val == 9 {
      None
    } else {
      Some(SudokuValue::new(val + 1))
    }
  }
}
