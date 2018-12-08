use super::checker::SudokuGridConflictChecker;
use super::core::{SudokuChoice, SudokuPosition, SudokuValue};

#[must_use]
#[derive(Clone, Copy)]
enum SolverProgressStatus {
  SolverCouldNotMakeProgress,
  SolverMadeProgress,
}

impl SolverProgressStatus {
  pub fn did_make_progress(self) -> bool {
    match self {
      SolverProgressStatus::SolverCouldNotMakeProgress => false,
      SolverProgressStatus::SolverMadeProgress => true,
    }
  }
}

pub struct SudokuSolver {
  choices: Vec<SudokuChoice>,
  fill_order: Vec<SudokuPosition>,
  grid_checker: SudokuGridConflictChecker,
}

impl SudokuSolver {
  pub fn solve(
    given_choices: &[SudokuChoice],
  ) -> Option<Vec<SudokuChoice>> {
    let fill_order: Vec<_> = SudokuPosition::all()
      .into_iter()
      .filter(|pos| {
        !given_choices
          .iter()
          .any(|given_choice| given_choice.position == *pos)
      }).collect();

    let mut grid_checker = SudokuGridConflictChecker::new();
    for &given_choice in given_choices {
      if !grid_checker.add_choice(given_choice).did_add() {
        panic!("Choices are manifestly incompatable.");
      }
    }

    let solver = SudokuSolver {
      choices: vec![],
      grid_checker,
      fill_order,
    };

    solver.run()
  }

  fn run(mut self) -> Option<Vec<SudokuChoice>> {
    loop {
      if self.is_complete() {
        return Some(self.choices);
      }

      // Try to fill next position.
      let next_position_to_fill = self.next_position_to_fill();
      if self
        .try_to_extend_solution(
          SudokuValue::first(),
          next_position_to_fill,
        ).did_make_progress()
      {
        // Great! Let's loop around and try to keep extending!
        continue;
      }

      // Else, we must backtrack. But backtracking may fail, in which
      // case we must report that we couldn't solve the Sudoku :-|.
      if !self.backtrack().did_make_progress() {
        return None;
      }
    }
  }

  // Try to extend solution at the specified position. `start_value`
  // tells the initial value to try; this lets us not retry previously
  // attempted values.
  fn try_to_extend_solution(
    &mut self,
    start_value: SudokuValue,
    position: SudokuPosition,
  ) -> SolverProgressStatus {
    let mut value = start_value;
    loop {
      let choice = SudokuChoice::new(position, value);

      // Check whether choice is okay with existing conflicts. If so,
      // save it to our choices and return.
      if self.grid_checker.add_choice(choice).did_add() {
        self.choices.push(choice);
        return SolverProgressStatus::SolverMadeProgress;
      }

      // Try the next possible value.
      value = match value.next() {
        // If we can't fill any value at this position let the caller
        // know.
        None => return SolverProgressStatus::SolverCouldNotMakeProgress,
        Some(new_value) => new_value,
      };
    }
  }

  fn backtrack(&mut self) -> SolverProgressStatus {
    loop {
      // If the choices stack is empty; it's game over. We exhausted all
      // possibilities.
      let prev_choice = match self.choices.pop() {
        None => return SolverProgressStatus::SolverCouldNotMakeProgress,
        Some(prev_choice) => prev_choice,
      };

      // We're "undoing" this choice, so we must clear the conflicts we
      // recorded for it.
      self.grid_checker.remove_choice(prev_choice);

      let next_value_to_try = match prev_choice.value.next() {
        // If we've exhausted possible values for this position, we must
        // keep on backtracking.
        None => continue,
        Some(next_value_to_try) => next_value_to_try,
      };

      // See if there is another value that works at this position.
      if self
        .try_to_extend_solution(next_value_to_try, prev_choice.position)
        .did_make_progress()
      {
        return SolverProgressStatus::SolverMadeProgress;
      }

      // If not, we keep unwinding the choices stack.
    }
  }

  fn is_complete(&self) -> bool {
    self.choices.len() == self.fill_order.len()
  }

  fn next_position_to_fill(&self) -> SudokuPosition {
    self.fill_order[self.choices.len()]
  }
}

// use rand::{self, prelude::*};
// fn shuffled_sudoku_positions() -> Vec<SudokuPosition> {
//   let mut rng = rand::thread_rng();
//   let mut sudoku_positions = SudokuPosition::all();
//   // sudoku_positions.shuffle(&mut rng);
//   sudoku_positions
// }
