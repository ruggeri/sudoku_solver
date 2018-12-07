extern crate sudoku_solving;

use sudoku_solving::{
  SudokuGrid,
  SudokuSolver
};

fn main() {
  println!("Hello, world!");

  let mut grid = SudokuGrid::default();

  let solution = SudokuSolver::solve(&[]).unwrap();

  for choice in solution {
    grid.place(choice);
  }

  grid.print();
}
