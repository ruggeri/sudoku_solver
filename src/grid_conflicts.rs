use super::cell_conflicts::SudokuGroupConflictChecker;
use super::choice::SudokuChoice;
use super::sudoku_box::SudokuBox;

pub enum AddChoiceResult {
  CouldNotAddChoice,
  DidAddChoice,
}

pub struct SudokuGridConflictChecker {
  cell_conflicts: [[SudokuGroupConflictChecker; 9]; 9],
}

impl SudokuGridConflictChecker {
  pub fn new() -> SudokuGridConflictChecker {
    let cell_conflicts = unsafe {
      use std::mem;
      use std::ptr;

      let mut cell_conflicts: [[SudokuGroupConflictChecker; 9]; 9] = mem::uninitialized();
      for row in &mut cell_conflicts {
        for cell_conflicts in row {
          ptr::write(cell_conflicts, SudokuGroupConflictChecker::new());
        }
      }

      cell_conflicts
    };

    SudokuGridConflictChecker {
      cell_conflicts,
    }
  }

  pub fn add_choice(&mut self, choice: SudokuChoice) -> AddChoiceResult {
    if !self.can_accomodate_conflicts(choice) {
      AddChoiceResult::CouldNotAddChoice
    } else {
      self.propagate_conflicts(choice);
      AddChoiceResult::DidAddChoice
    }
  }

  pub fn remove_choice(&mut self, choice: SudokuChoice) {
    self.remove_conflicts(choice);
  }

  #[allow(if_same_then_else)]
  fn propagate_conflicts(&mut self, choice: SudokuChoice) {
    let (choice_row_idx, choice_col_idx) = choice.position.as_usize_pair();

    for new_row_idx in 0..9 {
      if new_row_idx == choice_row_idx {
        continue;
      }

      self.cell_conflicts[new_row_idx][choice_col_idx].add_conflict(choice.value);
    }

    for new_col_idx in 0..9 {
      if new_col_idx == choice_col_idx {
        continue;
      }

      self.cell_conflicts[choice_row_idx][new_col_idx].add_conflict(choice.value);
    }

    SudokuBox::for_position(choice.position).positions().for_each(|box_pos| {
      let (box_pos_row_idx, box_pos_col_idx) = box_pos.as_usize_pair();

      if box_pos_row_idx == choice_row_idx {
        // Skip; handled above.
      } else if box_pos_col_idx == choice_col_idx {
        // Skip; handled above.
      } else {
        self.cell_conflicts[box_pos_row_idx][box_pos_col_idx].add_conflict(choice.value);
      }
    });
  }

  #[allow(if_same_then_else)]
  fn can_accomodate_conflicts(&self, choice: SudokuChoice) -> bool {
    let (choice_row_idx, choice_col_idx) = choice.position.as_usize_pair();

    if !self.cell_conflicts[choice_row_idx][choice_col_idx].can_store_here(choice.value) {
      return false;
    }

    for new_row_idx in 0..9 {
      if new_row_idx == choice_row_idx {
        continue;
      }

      let cell_conflict = &self.cell_conflicts[new_row_idx][choice_col_idx];
      if !cell_conflict.can_restrict_here(choice.value) {
        return false;
      }
    }

    for new_col_idx in 0..9 {
      if new_col_idx == choice_col_idx {
        continue;
      }

      let cell_conflict = &self.cell_conflicts[choice_row_idx][new_col_idx];
      if !cell_conflict.can_restrict_here(choice.value) {
        return false;
      }
    }

    SudokuBox::for_position(choice.position).positions().all(|box_pos| {
      let (box_pos_row_idx, box_pos_col_idx) = box_pos.as_usize_pair();

      if box_pos_row_idx == choice_row_idx {
        // Skip; handled above.
        true
      } else if box_pos_col_idx == choice_col_idx {
        // Skip; handled above.
        true
      } else {
        self.cell_conflicts[box_pos_row_idx][box_pos_col_idx].can_restrict_here(choice.value)
      }
    })
  }

  #[allow(if_same_then_else)]
  fn remove_conflicts(&mut self, choice: SudokuChoice) {
    let (choice_row_idx, choice_col_idx) = choice.position.as_usize_pair();

    for new_row_idx in 0..9 {
      if new_row_idx == choice_row_idx {
        continue;
      }

      self.cell_conflicts[new_row_idx][choice_col_idx].remove_conflict(choice.value);
    }

    for new_col_idx in 0..9 {
      if new_col_idx == choice_col_idx {
        continue;
      }

      self.cell_conflicts[choice_row_idx][new_col_idx].remove_conflict(choice.value);
    }

    SudokuBox::for_position(choice.position).positions().for_each(|box_pos| {
      let (box_pos_row_idx, box_pos_col_idx) = box_pos.as_usize_pair();

      if box_pos_row_idx == choice_row_idx {
        // Skip; handled above.
      } else if box_pos_col_idx == choice_col_idx {
        // Skip; handled above.
      } else {
        self.cell_conflicts[box_pos_row_idx][box_pos_col_idx].remove_conflict(choice.value);
      }
    });
  }
}

impl Default for SudokuGridConflictChecker {
  fn default() -> SudokuGridConflictChecker {
    Self::new()
  }
}
