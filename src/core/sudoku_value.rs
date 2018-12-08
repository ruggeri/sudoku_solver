pub use super::SUDOKU_DIM_U8;

// A SudokuValue is a number 1 through 9 (inclusive). Zero is *not* a
// valid Sudoku value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SudokuValue(u8);

impl SudokuValue {
  pub fn first() -> SudokuValue {
    SudokuValue(1)
  }

  pub fn new(val: u8) -> SudokuValue {
    assert!(1 <= val && val <= SUDOKU_DIM_U8);
    SudokuValue(val)
  }

  // `as_u8_value` returns the numeric value 1 through 9 (inclusive).
  // Should be used only for display.
  pub fn as_u8_value(self) -> u8 {
    let SudokuValue(val) = self;
    val
  }

  // `as_usize_idx` returns a usize value 0 through 8 (inclusive). The
  // intent is for the usize to be used as an index in an array.
  pub fn as_usize_idx(self) -> usize {
    (self.as_u8_value() as usize) - 1
  }

  // `next` is used so that the caller can iterate through SudokuValues.
  pub fn next(self) -> Option<SudokuValue> {
    let val = self.as_u8_value();

    if val == SUDOKU_DIM_U8 {
      None
    } else {
      Some(SudokuValue::new(val + 1))
    }
  }
}
