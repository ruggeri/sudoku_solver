use ::core::SudokuValue;

#[derive(Clone, Default)]
pub struct SudokuGroupConflictChecker {
  value_conflicts: [u8; 9],
  number_free_values: u8
}

impl SudokuGroupConflictChecker {
  pub fn new() -> SudokuGroupConflictChecker {
    SudokuGroupConflictChecker {
      value_conflicts: [0; 9],
      number_free_values: 9,
    }
  }

  pub fn add_conflict(&mut self, val: SudokuValue) {
    if self.value_conflicts[val.as_usize_idx()] == 0 {
      self.number_free_values -= 1;
    }

    self.value_conflicts[val.as_usize_idx()] += 1;
  }

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

  pub fn can_store_here(&self, val: SudokuValue) -> bool {
    self.value_conflicts[val.as_usize_idx()] == 0
  }

  pub fn remove_conflict(&mut self, val: SudokuValue) {
    self.value_conflicts[val.as_usize_idx()] -= 1;

    if self.value_conflicts[val.as_usize_idx()] == 0 {
      self.number_free_values += 1;
    }
  }

  pub fn is_unsatisfiable(&self) -> bool {
    self.number_free_values == 0
  }
}
