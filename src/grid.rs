use super::{
  choice::SudokuChoice,
  sudoku_cell::SudokuCell,
};

pub struct SudokuGrid {
  cells: [[SudokuCell; 9]; 9],
}

impl SudokuGrid {
  pub fn default() -> SudokuGrid {
    let cells = unsafe {
      use std::mem;
      use std::ptr;

      let mut cells: [[SudokuCell; 9]; 9] = mem::uninitialized();
      for row in cells.iter_mut() {
        for cell in row.iter_mut() {
          ptr::write(cell, SudokuCell::Empty);
        }
      }

      cells
    };

    SudokuGrid { cells }
  }

  pub fn place(&mut self, choice: SudokuChoice) {
    let (row_idx, col_idx) = choice.position.as_usize_pair();

    self.cells[row_idx][col_idx] = SudokuCell::Filled(choice.value);
  }

  pub fn print(&self) {
    for row in &self.cells {
      for cell in row {
        match cell {
          SudokuCell::Empty => print!("."),
          SudokuCell::Filled(value) => print!("{}", value.as_u8_val())
        }
      }

      println!();
    }
  }

  // pub fn valid_row(&self, idx: usize) -> SudokuStatus {
  //   let mut checker = ValidityChecker::new();
  //   for &cell in &self.cells[idx] {
  //     checker.check_cell(cell)
  //   }

  //   checker.status()
  // }

  // pub fn valid_column(&self, col_idx: usize) -> SudokuStatus {
  //   let mut checker = ValidityChecker::new();
  //   for row_idx in 0..9 {
  //     checker.check_cell(self.cells[row_idx][col_idx]);
  //   }

  //   checker.status()
  // }

  // pub fn valid_box(&self, sb: SudokuBox) -> SudokuStatus {
  //   let mut checker = ValidityChecker::new();

  //   let BoxCoord(box_x, box_y) = sb.to_box_coord();

  //   for rel_x in 0..3 {
  //     let abs_x = (box_x*3 + rel_x) as usize;
  //     for rel_y in 0..3 {
  //       let abs_y = (box_y*3 + rel_y) as usize;
  //       checker.check_cell(self.cells[abs_x][abs_y]);
  //     }
  //   }

  //   checker.status()
  // }
}
