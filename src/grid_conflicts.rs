use std::iter;
use super::cell_conflicts::SudokuGroupConflictChecker;
use super::choice::SudokuChoice;
use super::position::SudokuPosition;
use super::sudoku_box::SudokuBox;

pub enum AddChoiceResult {
  CouldNotAddChoice,
  DidAddChoice,
}

pub struct SudokuGridConflictChecker {
  cell_conflicts: [[SudokuGroupConflictChecker; 9]; 9],
  box_conflicts: [SudokuGroupConflictChecker; 9],
}

impl SudokuGridConflictChecker {
  pub fn new() -> SudokuGridConflictChecker {
    let (cell_conflicts, box_conflicts) = unsafe {
      use std::mem;
      use std::ptr;

      let mut cell_conflicts: [[SudokuGroupConflictChecker; 9]; 9] = mem::uninitialized();
      let mut box_conflicts: [SudokuGroupConflictChecker; 9] = mem::uninitialized();
      for row in cell_conflicts.iter_mut() {
        for cell_conflicts in row.iter_mut() {
          ptr::write(cell_conflicts, SudokuGroupConflictChecker::new());
        }
      }

      for box_conflict in box_conflicts.iter_mut() {
        ptr::write(box_conflict, SudokuGroupConflictChecker::new());
      }

      (cell_conflicts, box_conflicts)
    };

    SudokuGridConflictChecker {
      cell_conflicts,
      box_conflicts,
    }
  }

  pub fn add_choice(&mut self, choice: SudokuChoice) -> AddChoiceResult {
    if !self.can_accomodate_conflicts(choice) {
      AddChoiceResult::CouldNotAddChoice
    } else {
      self.add_conflicts(choice);
      AddChoiceResult::DidAddChoice
    }
  }

  pub fn remove_choice(&mut self, choice: SudokuChoice) {
    self.remove_conflicts(choice);
  }

  fn add_conflicts(&mut self, choice: SudokuChoice) {
    let (row_idx, col_idx) = choice.position.as_usize_pair();

    for new_row_idx in 0..9 {
      self.cell_conflicts[new_row_idx][col_idx].add_conflict(choice.value);
    }

    for new_col_idx in 0..9 {
      self.cell_conflicts[row_idx][new_col_idx].add_conflict(choice.value);
    }

    let box_idx = SudokuBox::for_position(choice.position).to_usize();
    self.box_conflicts[box_idx].add_conflict(choice.value);
  }

  fn can_accomodate_conflicts(&self, choice: SudokuChoice) -> bool {
    let (row_idx, col_idx) = choice.position.as_usize_pair();

    for new_row_idx in 0..9 {
      let cell_conflict = &self.cell_conflicts[new_row_idx][col_idx];
      if !cell_conflict.can_accomodate_conflict(choice.value) {
        return false
      }
    }

    for new_col_idx in 0..9 {
      let cell_conflict = &self.cell_conflicts[row_idx][new_col_idx];
      if !cell_conflict. can_accomodate_conflict(choice.value) {
        return false
      }
    }

    let box_idx = SudokuBox::for_position(choice.position).to_usize();
    self.box_conflicts[box_idx].can_accomodate_conflict(choice.value)
  }

  fn remove_conflicts(&mut self, choice: SudokuChoice) {
    let (row_idx, col_idx) = choice.position.as_usize_pair();

    for new_row_idx in 0..9 {
      self.cell_conflicts[new_row_idx][col_idx].remove_conflict(choice.value);
    }

    for new_col_idx in 0..9 {
      self.cell_conflicts[row_idx][new_col_idx].remove_conflict(choice.value);
    }

    let box_idx = SudokuBox::for_position(choice.position).to_usize();
    self.box_conflicts[box_idx].remove_conflict(choice.value);
  }
}
