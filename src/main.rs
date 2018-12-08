extern crate sudoku_solving;

use sudoku_solving::{
  core::{SudokuChoice, SudokuGrid, SudokuPosition, SudokuValue},
  solver::SudokuSolver,
};

fn main() {
  println!("Hello, world!");

  let mut grid = SudokuGrid::default();

  let filled_choices = vec![SudokuChoice::new(
    SudokuPosition::new(8, 8),
    SudokuValue::new(9),
  )];

  let solution = SudokuSolver::solve(&filled_choices).unwrap();

  for choice in solution {
    grid.place(choice);
  }

  print!("{}", grid);
}
