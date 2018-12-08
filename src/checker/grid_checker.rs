use super::SudokuGroupConflictChecker;
use core::{SudokuBox, SudokuChoice, SUDOKU_DIM_USIZE};

// SudokuGridConflictChecker builds on top of
// SudokuGroupConflictChecker. It checks whether a choice of value at a
// given position is compatible with constraints on all the other values
// in the same row/column/box.
#[derive(Default)]
pub struct SudokuGridConflictChecker {
  // Keeps a 9x9 grid of group checkers.
  cell_checkers:
    [[SudokuGroupConflictChecker; SUDOKU_DIM_USIZE]; SUDOKU_DIM_USIZE],
}

impl SudokuGridConflictChecker {
  pub fn new() -> SudokuGridConflictChecker {
    // A SudokuGroupConflictChecker is not Copy, so it is a little
    // annoying to initialize a array of checkers.
    let cell_checkers = unsafe {
      use std::mem;
      use std::ptr;

      // If there were a panic, then it uninitialized memory could leak
      // out. Whatever...
      let mut cell_checkers: [[SudokuGroupConflictChecker;
                               SUDOKU_DIM_USIZE];
                               SUDOKU_DIM_USIZE] = mem::uninitialized();

      for row in &mut cell_checkers {
        for chell_checker in row {
          ptr::write(chell_checker, SudokuGroupConflictChecker::new());
        }
      }

      cell_checkers
    };

    SudokuGridConflictChecker { cell_checkers }
  }

  // `add_choice` records the choice, updating all the relevant checkers
  // in the same row/column/box. However, if the choice of value is
  // either (1) invalid at the specified position, or (2) would
  // "overconstrain" other cells in the same row/column/box so they have
  // no remaining possible value, then we reject the choice.
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

  // `remove_choice` updates the cells in the same row/column/box to,
  // removing the constraint imposed on them by the choice of value
  // here.
  pub fn remove_choice(&mut self, choice: SudokuChoice) {
    self.unpropagate_conflicts(choice);
  }

  // `can_accomodate_choice` checks first whether the choice is valid at
  // the specified position, and that it doesn't "overconstrain" other
  // values in the same row/column/box.
  #[allow(if_same_then_else)]
  fn can_accomodate_choice(&self, choice: SudokuChoice) -> bool {
    let (choice_row_idx, choice_col_idx) =
      choice.position.as_usize_pair();

    // If the choice is invalid to store here, then this is bogus.
    if !self.cell_checkers[choice_row_idx][choice_col_idx]
      .can_store_here(choice.value)
    {
      return false;
    }

    // Check other cells in the same row.
    for new_row_idx in 0..SUDOKU_DIM_USIZE {
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

    // Check other cells in the same column.
    for new_col_idx in 0..SUDOKU_DIM_USIZE {
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

    // Check other cells in the same box.
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

    // I've decided not to "add_conflict" at the position where we are
    // setting this value. The checker for this position shouldn't be in
    // "conflict" with its own chosen value; that would be perverse.

    // Propagate constraints to cells in the same row.
    for new_row_idx in 0..SUDOKU_DIM_USIZE {
      if new_row_idx == choice_row_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[new_row_idx][choice_col_idx]
        .add_conflict(choice.value);
    }

    // Propagate constraints to cells in the same column.
    for new_col_idx in 0..SUDOKU_DIM_USIZE {
      if new_col_idx == choice_col_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[choice_row_idx][new_col_idx]
        .add_conflict(choice.value);
    }

    // Propagate constraints to cells in the same box.
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

    // See not above in `propagate_conflicts` for why I don't need to
    // `remove_conflict` on the chosen position we are undoing.

    // Remove constraints to cells in the same row.
    for new_row_idx in 0..SUDOKU_DIM_USIZE {
      if new_row_idx == choice_row_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[new_row_idx][choice_col_idx]
        .remove_conflict(choice.value);
    }

    // Remove constraints to cells in the same column.
    for new_col_idx in 0..SUDOKU_DIM_USIZE {
      if new_col_idx == choice_col_idx {
        // Skip; already discussed above.
        continue;
      }

      self.cell_checkers[choice_row_idx][new_col_idx]
        .remove_conflict(choice.value);
    }

    // Remove constraints to cells in the same box.
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

// AddChoiceResult will tell the caller of `add_choice` whether their
// choice could be made. The caller must not ignore this return value,
// as their choice may have been impossible (and therefore rejected).
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
