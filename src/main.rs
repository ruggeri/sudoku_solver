extern crate sudoku_solving;

use sudoku_solving::{
  core::{SudokuChoice, SudokuGrid, SudokuPosition, SudokuValue},
  solver::SudokuSolver,
};

fn main() {
  // Some pre-filled values for the Sudoku grid.
  let filled_choices = vec![SudokuChoice::new(
    SudokuPosition::new(8, 8),
    SudokuValue::new(9),
  )];

  // Try to solve the grid.
  let solution = SudokuSolver::solve(&filled_choices).unwrap();

  // Write the solution choices into a grid, and print it.
  let mut grid = SudokuGrid::default();
  for choice in filled_choices {
    grid.place(choice);
  }
  for choice in solution {
    grid.place(choice);
  }

  print!("{}", grid);
}
