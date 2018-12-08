mod sudoku_box;
mod sudoku_cell;
mod sudoku_choice;
mod sudoku_grid;
mod sudoku_position;
mod sudoku_value;

pub const SUDOKU_DIM_SQRT_U8: u8 = 3;
pub const SUDOKU_DIM_SQRT_USIZE: usize = SUDOKU_DIM_SQRT_U8 as usize;
pub const SUDOKU_DIM_U8: u8 = SUDOKU_DIM_SQRT_U8 * SUDOKU_DIM_SQRT_U8;
pub const SUDOKU_DIM_USIZE: usize =
  SUDOKU_DIM_SQRT_USIZE * SUDOKU_DIM_SQRT_USIZE;

pub use self::sudoku_box::SudokuBox;
pub use self::sudoku_cell::SudokuCell;
pub use self::sudoku_choice::SudokuChoice;
pub use self::sudoku_grid::SudokuGrid;
pub use self::sudoku_position::SudokuPosition;
pub use self::sudoku_value::SudokuValue;
