// use super::{
//   checker::{
//     SudokuStatus,
//     ValidityChecker,
//   },
//   sudoku_box::{
//     BoxCoord,
//     SudokuBox,
//   }
// };

// pub struct SudokuGrid {
//   cells: [[SudokuCell; 9]; 9],
// }

// impl SudokuGrid {
//   pub fn valid_row(&self, idx: usize) -> SudokuStatus {
//     let mut checker = ValidityChecker::new();
//     for &cell in &self.cells[idx] {
//       checker.check_cell(cell)
//     }

//     checker.status()
//   }

//   pub fn valid_column(&self, col_idx: usize) -> SudokuStatus {
//     let mut checker = ValidityChecker::new();
//     for row_idx in 0..9 {
//       checker.check_cell(self.cells[row_idx][col_idx]);
//     }

//     checker.status()
//   }

//   pub fn valid_box(&self, sb: SudokuBox) -> SudokuStatus {
//     let mut checker = ValidityChecker::new();

//     let BoxCoord(box_x, box_y) = sb.to_box_coord();

//     for rel_x in 0..3 {
//       let abs_x = (box_x*3 + rel_x) as usize;
//       for rel_y in 0..3 {
//         let abs_y = (box_y*3 + rel_y) as usize;
//         checker.check_cell(self.cells[abs_x][abs_y]);
//       }
//     }

//     checker.status()
//   }
// }
