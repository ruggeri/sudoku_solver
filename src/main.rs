extern crate sudoku_solving;

use sudoku_solving::SudokuSolver;

fn main() {
  println!("Hello, world!");
  let solution = SudokuSolver::solve(&[]);
  println!("Solution: {:?}", solution);
  println!("Solution len: {}", solution.unwrap().len());
}
