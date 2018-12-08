use core::SudokuValue;

// A SudokGroupConflictChecker keeps track of what values are available
// for a given cell in the Sudoku grid. This allows the user to quickly
// determine whether a choice is valid at a given position.
#[derive(Clone, Default)]
pub struct SudokuGroupConflictChecker {
  // At position `i`, `value_conflicts[i]` is the number of cells that
  // conflict with the assignment of value `i + 1` here (note `i + 1` is
  // because SudokuValue ranges from 1 through 9 inclusive).
  value_conflicts: [u8; 9],
  // The number of values that the cell may still possibly take on. When
  // this is zero then there is no possibly valid choice here. In that
  // case, we must have made bad prior choices.
  number_free_values: u8,
}

impl SudokuGroupConflictChecker {
  pub fn new() -> SudokuGroupConflictChecker {
    // In the beginning, the cell is free of any conflicts, and can take
    // on any value.
    SudokuGroupConflictChecker {
      value_conflicts: [0; 9],
      number_free_values: 9,
    }
  }

  // Record a SudokuValue that constrains our value here. For isntance,
  // val may have been placed in the same row/column/box.
  pub fn add_conflict(&mut self, val: SudokuValue) {
    // If this is the first conflict for the given value, we have one
    // less possible choice here.
    if self.value_conflicts[val.as_usize_idx()] == 0 {
      self.number_free_values -= 1;
    }

    self.value_conflicts[val.as_usize_idx()] += 1;
  }

  // Will the given choice elsewhere in the same row/col/box eliminate
  // the last possible value here?
  #[allow(if_same_then_else, needless_bool)]
  pub fn can_restrict_here(&self, val: SudokuValue) -> bool {
    // Why are you asking if you can further restrict here if it is
    // already too restricted?
    assert!(self.number_free_values >= 1);

    if self.value_conflicts[val.as_usize_idx()] > 0 {
      // Doesn't change number of free values.
      true
    } else if self.number_free_values > 1 {
      // Won't restrict number of free values to zero.
      true
    } else {
      // We would be restricting our last remaining option.
      false
    }
  }

  // Does storing `val` here conflict with any prior recorded conflict.
  // Note how this is different from `can_restrict_here`.
  //
  // For instance, it is fine if `val` is the last valid value here.
  pub fn can_store_here(&self, val: SudokuValue) -> bool {
    self.value_conflicts[val.as_usize_idx()] == 0
  }

  // When you "undo" a choice, remove the constraint here.
  pub fn remove_conflict(&mut self, val: SudokuValue) {
    self.value_conflicts[val.as_usize_idx()] -= 1;

    // If this was the last choice forbidding the given value, then we
    // increase the options of values we can place here.
    if self.value_conflicts[val.as_usize_idx()] == 0 {
      self.number_free_values += 1;
    }
  }
}
