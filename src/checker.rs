// use super::{
//   grid::SudokuCell
// };

// #[derive(Clone, Copy)]
// pub enum SudokuValidity {
//   Invalid,
//   Valid
// }

// #[derive(Clone, Copy)]
// pub struct SudokuStatus(CompletionStatus, SudokuValidity);

// #[derive(Clone, Copy, Eq, PartialEq)]
// pub enum NumberUsage {
//   InUse,
//   NotInUse
// }

// pub struct ValidityChecker {
//   number_is_used: [NumberUsage; 9],
//   status: SudokuStatus,
// }

// impl ValidityChecker {
//   pub fn new() -> ValidityChecker {
//     ValidityChecker {
//       number_is_used: [NumberUsage::NotInUse; 9],
//       status: SudokuStatus(CompletionStatus::Complete, SudokuValidity::Valid),
//     }
//   }

//   pub fn check_cell(&mut self, cell: SudokuCell) {
//     let val = match cell {
//       SudokuCell::Empty => {
//         self.status.0 = CompletionStatus::Incomplete;
//         return
//       }

//       SudokuCell::Filled(val) => val
//     };

//     match self.number_is_used[val.as_usize()] {
//       NumberUsage::NotInUse => {
//         self.number_is_used[val.as_usize()] = NumberUsage::InUse;
//       }

//       NumberUsage::InUse => {
//         self.status.1 = SudokuValidity::Invalid;
//       }
//     }
//   }

//   pub fn status(&self) -> SudokuStatus {
//     self.status
//   }
// }
