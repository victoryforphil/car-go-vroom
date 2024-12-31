use std::{borrow::BorrowMut, cell::Cell};


#[derive(Default, Clone, Copy, Debug)]
pub enum CellValue{
    Cross = -1,
    #[default]
    Empty = 0,
    Circle = 1
}

const BOARD_SIZE: usize = 3; // 3x3 Grid

#[derive(Default, Clone, Debug)]
pub struct Board{
    cells: [CellValue; BOARD_SIZE * BOARD_SIZE] // Represents a Row major X/Y grid of size 3x3
}

impl Board{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn calc_index(x: usize, y: usize) -> Result<usize, String>{
        // Row = Y
        // Col = X
        let width = BOARD_SIZE;

        if x > width || y > width {
            return Err(format!("IndexOutBounds for Board of size {}, value: {}, {}", width, x, y));
        }
        return Ok( y * width + x);
    }

    pub fn calc_xy(i: usize) -> Result<(usize, usize), String>{
        // Row = y
        // Col = x
        let y = i / BOARD_SIZE;
        let x = i % BOARD_SIZE;

        return Ok((x, y))

    }
    
    pub fn get_cell_ref(&self, x: usize, y: usize) -> Result<&CellValue, String>{
        let idx = Self::calc_index(x, y)?;
        Ok(&self.cells[idx])
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Result<&mut CellValue, String>{
        let idx = Self::calc_index(x, y)?;
        Ok(&mut self.cells[idx])
    }


    pub fn get_col_mut(&mut self, x: usize) -> Result<Vec<&mut CellValue>, String>{
        let iter_mut: std::slice::IterMut<'_, CellValue> = self.cells.iter_mut();
        Ok(iter_mut.into_iter().skip(x).step_by(3).collect())
    }

    pub fn get_row_mut(&mut self, y: usize) -> Result<Vec<&mut CellValue>, String>{
        let iter_mut: std::slice::IterMut<'_, CellValue> = self.cells.iter_mut();
        Ok(iter_mut.into_iter().skip(y * BOARD_SIZE).take(BOARD_SIZE).collect())
    }

    pub fn draw_board_flattened(&self) -> Result<(), String>{
        for i in 0..self.cells.len(){ // BOARD_SIZE
           
           let (x,y) = Self::calc_xy(i)?;
         

           if x == 0{
                print!("\n");
           }
           let cell_value = self.get_cell_ref(x,y)?;
           let char = match cell_value{
            CellValue::Empty => "*",
            CellValue::Circle => "O",
            CellValue::Cross => "X"
           };
           print!("{}", char);
            
        }

        println!("");

        Ok(())
    }
}

pub fn main(){
    let mut board = Board::new();

    board.draw_board_flattened();

    let mut cell = board.get_cell_mut(1,2).expect("Failed to get cell");
    *cell = CellValue::Cross;


    board.draw_board_flattened();

    let mut row = board.get_row_mut(0).unwrap();

    for cell in row{
        *cell = CellValue::Circle;
    }

    

    board.draw_board_flattened();

    let mut col = board.get_col_mut(0).unwrap();

    for cell in col{
        *cell = CellValue::Circle;
    }

    board.draw_board_flattened();
}