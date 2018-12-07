use rand::{
  prelude::*,
  self,
};
use super::{
  choice::SudokuChoice,
  grid_conflicts::{AddChoiceResult, SudokuGridConflictChecker},
  position::SudokuPosition,
  value::SudokuValue,
};

pub struct SudokuSolver {
  choices: Vec<SudokuChoice>,
  grid_checker: SudokuGridConflictChecker,
  fill_order: Vec<SudokuPosition>
}

impl SudokuSolver {
  pub fn solve(given_choices: &[SudokuChoice]) -> Option<Vec<SudokuChoice>> {
    let fill_order: Vec<_> = shuffled_sudoku_positions()
      .into_iter()
      .filter(|pos| !given_choices.iter().any(|given_choice| given_choice.position == *pos))
      .collect();

    let solver = SudokuSolver {
      choices: Vec::new(),
      grid_checker: SudokuGridConflictChecker::new(),
      fill_order,
    };

    solver.run()
  }

  fn run(mut self) -> Option<Vec<SudokuChoice>> {
    loop {
      if self.is_complete() {
        return Some(self.choices);
      }

      if !self.run_next() {
        return None;
      }
    }
  }

  fn run_next(&mut self) -> bool {
    // Try to fill next position.
    let next_position_to_fill = self.next_position_to_fill();
    if self.try_to_fill(SudokuValue::first(), next_position_to_fill) {
      return true;
    }

    // If we could not, we must backtrack. Backtracking may fail, in
    // which case game over.
    self.backtrack()
  }

  fn is_complete(&self) -> bool {
    self.choices.len() == self.fill_order.len()
  }

  fn next_position_to_fill(&self) -> SudokuPosition {
    self.fill_order[self.choices.len()]
  }

  fn try_to_fill(&mut self, start_value: SudokuValue, position: SudokuPosition) -> bool {
    let mut value = start_value;
    loop {
      let choice = SudokuChoice::new(position, value);

      if let AddChoiceResult::DidAddChoice = self.grid_checker.add_choice(choice) {
        self.choices.push(choice);
        return true
      }

      value = match value.next() {
        None => break,
        Some(new_value) => new_value,
      };
    }

    // Wasn't able to fill this position.
    false
  }

  fn backtrack(&mut self) -> bool {
    loop {
      let last_choice = match self.choices.pop() {
        None => return false,
        Some(last_choice) => last_choice
      };

      self.grid_checker.remove_choice(last_choice);

      let next_value = match last_choice.value.next() {
        // Keep on backtracking.
        None => continue,
        Some(next_value) => next_value,
      };

      if self.try_to_fill(next_value, last_choice.position) {
        return true
      }
    }
  }
}

fn shuffled_sudoku_positions() -> Vec<SudokuPosition> {
  let mut rng = rand::thread_rng();
  let mut sudoku_positions = SudokuPosition::all();
  // sudoku_positions.shuffle(&mut rng);
  sudoku_positions
}
