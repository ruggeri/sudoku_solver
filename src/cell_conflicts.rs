use super::value::SudokuValue;

#[derive(Clone)]
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
    if self.value_conflicts[val.as_idx()] == 0 {
      self.number_free_values -= 1;
    }

    self.value_conflicts[val.as_idx()] += 1;
  }

  #[allow(if_same_then_else)]
  pub fn can_accomodate_conflict(&self, val: SudokuValue) -> bool {
    assert!(self.number_free_values >= 1);

    if self.value_conflicts[val.as_idx()] > 0 {
      // Doesn't change number of free values.
      true
    } else if self.number_free_values > 1 {
      // Won't restrict number of free values to zero.
      true
    } else {
      false
    }
  }

  pub fn remove_conflict(&mut self, val: SudokuValue) {
    self.value_conflicts[val.as_idx()] -= 1;

    if self.value_conflicts[val.as_idx()] == 0 {
      self.number_free_values += 1;
    }
  }

  pub fn is_unsatisfiable(&self) -> bool {
    self.number_free_values == 0
  }
}
