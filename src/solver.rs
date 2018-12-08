use super::checker::SudokuGridConflictChecker;
use super::core::{SudokuChoice, SudokuPosition, SudokuValue};

// SudokuSolver implements a backtracking search to attempt to solve the
// Sudoku puzzle.
pub struct SudokuSolver {
  // `choices` is a vector of the solver's choices for the grid. Note: it
  // does *not* contain the pre-filled values chosen for us.
  choices: Vec<SudokuChoice>,
  // `fill_order` is the order in which we will try to fill values.
  fill_order: Vec<SudokuPosition>,
  // `grid_checker` keeps tracks of the conflicts your choices (and the
  // pre-filled values), impose.
  grid_checker: SudokuGridConflictChecker,
}

impl SudokuSolver {
  // `solve` builds the SudokuSolver and executes the backtracking
  // search. `given_choices` are the values that are prefilled in the
  // grid.
  //
  // Returns an `Option<Vec<SudokuChoice>>` because the grid may not be
  // satisfiable.
  pub fn solve(
    given_choices: &[SudokuChoice],
  ) -> Option<Vec<SudokuChoice>> {
    // `fill_order` is the left-to-right, top-to-bottom order of all
    // positions. We filter away the pre-filled values since we don't
    // get to choose those.
    //
    // I originally thought we might want to fill in a shuffled order,
    // so I made it easy to later change the fill order. But that is
    // silly and inefficient. We could probably get rid of this.
    //
    // It probably *could* be a good optimization to always fill next
    // the most constrained value. But that would definitely add
    // complications to bookkeeping (remembering what choices we have
    // previously tried).
    let fill_order: Vec<_> = SudokuPosition::all()
      .into_iter()
      .filter(|pos| {
        !given_choices
          .iter()
          .any(|given_choice| given_choice.position == *pos)
      }).collect();

    // This builds the `grid_checker`, and adds the constraints imposed
    // by the pre-filled values. If the grid_checker right away knows
    // the grid is unsatisfiable, we panic.
    let mut grid_checker = SudokuGridConflictChecker::new();
    for &given_choice in given_choices {
      if !grid_checker.add_choice(given_choice).did_add() {
        panic!("given_choices are manifestly incompatable.");
      }
    }

    let solver = SudokuSolver {
      choices: vec![],
      grid_checker,
      fill_order,
    };

    // Execute the solving routine.
    solver.run()
  }

  // `run` executes the backtracking search.
  fn run(mut self) -> Option<Vec<SudokuChoice>> {
    loop {
      // If we've filled all the cells, we have a solution!
      if self.is_complete() {
        return Some(self.choices);
      }

      // Try to fill next position, extending our previous choices.
      let next_position_to_fill = self.next_position_to_fill();
      if self
        .try_to_extend_solution(
          SudokuValue::first(),
          next_position_to_fill,
        ).did_make_progress()
      {
        // If we make progress, great! Let's loop around and try to keep
        // extending!
        continue;
      }

      // Else, there is no valid choice that remains for the
      // `next_position_to_fill`. We must backtrack.
      //
      // But backtracking may also fail, in which case we must report
      // that the Sudoku grid has no solution :-(
      if !self.backtrack().did_make_progress() {
        return None;
      }
    }
  }

  // `try_to_extend_solution` tries to extend the solution at the
  // specified position. `start_value` tells the initial value to try;
  // this lets us not retry previously attempted values.
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

      // If `value` didn't work, then let's try the next value.
      value = match value.next() {
        // If we exhausted all possible values at this position let the
        // caller know.
        None => return SolverProgressStatus::SolverCouldNotMakeProgress,
        Some(new_value) => new_value,
      };
    }
  }

  // `backtrack` undoes prior choices. We backtrack when we hit a
  // dead-end and can't advance otherwise.
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

// Methods may return `SolverProgressStatus` so that caller cannot
// ignore whether the method was able to make progress in making new
// SudokuChoices.
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

// use rand::{self, prelude::*};
// fn shuffled_sudoku_positions() -> Vec<SudokuPosition> {
//   let mut rng = rand::thread_rng();
//   let mut sudoku_positions = SudokuPosition::all();
//   // sudoku_positions.shuffle(&mut rng);
//   sudoku_positions
// }
