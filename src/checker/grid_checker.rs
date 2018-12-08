use super::SudokuGroupConflictChecker;
use core::{SudokuBox, SudokuChoice};

#[must_use]
#[derive(Clone, Copy)]
pub enum AddChoiceResult {
  CouldNotAddChoice,
  DidAddChoice,
}

impl AddChoiceResult {
  pub fn did_add(self) -> bool {
    match self {
      AddChoiceResult::CouldNotAddChoice => false,
      AddChoiceResult::DidAddChoice => true,
    }
  }
}

#[derive(Default)]
pub struct SudokuGridConflictChecker {
  cell_checkers: [[SudokuGroupConflictChecker; 9]; 9],
}

impl SudokuGridConflictChecker {
  pub fn new() -> SudokuGridConflictChecker {
    let cell_checkers = unsafe {
      use std::mem;
      use std::ptr;

      let mut cell_checkers: [[SudokuGroupConflictChecker; 9]; 9] =
        mem::uninitialized();

      for row in &mut cell_checkers {
        for chell_checker in row {
          ptr::write(chell_checker, SudokuGroupConflictChecker::new());
        }
      }

      cell_checkers
    };

    SudokuGridConflictChecker { cell_checkers }
  }

  pub fn add_choice(
    &mut self,
    choice: SudokuChoice,
  ) -> AddChoiceResult {
    if self.can_accomodate_choice(choice) {
      self.propagate_conflicts(choice);
      AddChoiceResult::DidAddChoice
    } else {
      AddChoiceResult::CouldNotAddChoice
    }
  }

  pub fn remove_choice(&mut self, choice: SudokuChoice) {
    self.unpropagate_conflicts(choice);
  }

  #[allow(if_same_then_else)]
  fn can_accomodate_choice(&self, choice: SudokuChoice) -> bool {
    let (choice_row_idx, choice_col_idx) =
      choice.position.as_usize_pair();

    if !self.cell_checkers[choice_row_idx][choice_col_idx]
      .can_store_here(choice.value)
    {
      return false;
    }

    for new_row_idx in 0..9 {
      if new_row_idx == choice_row_idx {
        // Skip; already checked that we can store here.
        continue;
      }

      let chell_checker =
        &self.cell_checkers[new_row_idx][choice_col_idx];
      if !chell_checker.can_restrict_here(choice.value) {
        return false;
      }
    }

    for new_col_idx in 0..9 {
      if new_col_idx == choice_col_idx {
        // Skip; already checked that we can store here.
        continue;
      }

      let chell_checker =
        &self.cell_checkers[choice_row_idx][new_col_idx];
      if !chell_checker.can_restrict_here(choice.value) {
        return false;
      }
    }

    SudokuBox::for_position(choice.position).positions().all(
      |box_pos| {
        let (box_pos_row_idx, box_pos_col_idx) =
          box_pos.as_usize_pair();

        if box_pos_row_idx == choice_row_idx {
          // Skip; handled above.
          true
        } else if box_pos_col_idx == choice_col_idx {
          // Skip; handled above.
          true
        } else {
          self.cell_checkers[box_pos_row_idx][box_pos_col_idx]
            .can_restrict_here(choice.value)
        }
      },
    )
  }

  #[allow(if_same_then_else)]
  fn propagate_conflicts(&mut self, choice: SudokuChoice) {
    let (choice_row_idx, choice_col_idx) =
      choice.position.as_usize_pair();

    // I've decide not to "add_conflict" at the position where we are
    // setting this value.

    for new_row_idx in 0..9 {
      if new_row_idx == choice_row_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[new_row_idx][choice_col_idx]
        .add_conflict(choice.value);
    }

    for new_col_idx in 0..9 {
      if new_col_idx == choice_col_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[choice_row_idx][new_col_idx]
        .add_conflict(choice.value);
    }

    SudokuBox::for_position(choice.position)
      .positions()
      .for_each(|box_pos| {
        let (box_pos_row_idx, box_pos_col_idx) =
          box_pos.as_usize_pair();

        if box_pos_row_idx == choice_row_idx {
          // Skip; already discussed above.
        } else if box_pos_col_idx == choice_col_idx {
          // Skip; already discussed above.
        } else {
          self.cell_checkers[box_pos_row_idx][box_pos_col_idx]
            .add_conflict(choice.value);
        }
      });
  }

  #[allow(if_same_then_else)]
  fn unpropagate_conflicts(&mut self, choice: SudokuChoice) {
    let (choice_row_idx, choice_col_idx) =
      choice.position.as_usize_pair();

    // I've decide not to "add_conflict" at the position where we are
    // setting this value. And thus I don't have to remove a conflict
    // for there.

    for new_row_idx in 0..9 {
      if new_row_idx == choice_row_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[new_row_idx][choice_col_idx]
        .remove_conflict(choice.value);
    }

    for new_col_idx in 0..9 {
      if new_col_idx == choice_col_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[choice_row_idx][new_col_idx]
        .remove_conflict(choice.value);
    }

    SudokuBox::for_position(choice.position)
      .positions()
      .for_each(|box_pos| {
        let (box_pos_row_idx, box_pos_col_idx) =
          box_pos.as_usize_pair();

        if box_pos_row_idx == choice_row_idx {
          // Skip; already discussed above.
        } else if box_pos_col_idx == choice_col_idx {
          // Skip; already discussed above.
        } else {
          self.cell_checkers[box_pos_row_idx][box_pos_col_idx]
            .remove_conflict(choice.value);
        }
      });
  }
}
