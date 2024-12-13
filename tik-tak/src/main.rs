use std::{io::{self, Write}, u8};

use anyhow::Error;


#[derive(Default, Clone, Copy)]
pub enum CellState{
    #[default]
    Empty = 0,
    CrossTaken = -1,
    CircleTaken = 1
}

pub struct Board{
    pub cells: [[CellState; 3];3]
}

impl Board{
    pub fn new() -> Self{
        Self{
            cells: [[CellState::default();3];3]
        }
    }
    pub fn clear_board(&self){
        #[cfg(target_os = "windows")]
        {
            // Clear screen on Windows
            let mut cmd = Command::new("cmd");
            cmd.args(&["/C", "cls"]);
            cmd.spawn().unwrap().wait().unwrap();
        }
        #[cfg(not(target_os = "windows"))]
        {
            // Clear screen on other systems (Linux, macOS, etc.)
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().unwrap();
        }
    }
    pub fn print_board(&self){

        // Print Coordinate helpers

        println!(" * |0| 1| 2|");

        for (row_num,row) in self.cells.iter().enumerate(){
            print!("|{}|", row_num);
            for cell in row{
                let to_print = match cell{
                    CellState::Empty => "[ ]",
                    CellState::CrossTaken => "[X]",
                    CellState::CircleTaken => "[O]",
                };
                print!("{}", to_print);
            }

            println!("");
        }
    }

    pub fn did_win(&self) -> bool{
        let mut found_win=  false;


        for (row_i, row) in self.cells.iter().enumerate(){
            let row_sum: isize = row.iter().map(|&s| s as isize).sum();
        
            if row_sum.abs() == 3{
                println!("FOUND WIN ON ROW: {:?}", row_i);
                found_win = true;
            }

        }

       
        let mut col_sums = [0; 3];


        for col in 0..3 {
            for row in 0..3{
                let cell = self.cells[row][col];
                col_sums[col] += cell as i32 
            }
        }

        if col_sums.iter().any(|col_sum| {col_sum.abs() == 3}){
            println!("Found win on COL!");
            found_win = true;
        }

        // Cros check

        let mut cross_sum = 0;
        for cell_i in 0..3{
            cross_sum =  cross_sum+ self.cells[cell_i][cell_i] as i32;
        }

        let mut cross_sum_rev = 0;
        for cell_i in 0..3{
            cross_sum_rev = cross_sum_rev +self.cells[2-cell_i][cell_i] as i32;
        }
      

      if cross_sum_rev.abs() == 3 || cross_sum.abs() == 3{
        println!("CROSS WON");
            found_win = true;
      }

        found_win

        
    }

    pub fn parse_input(&mut self, input_string: &String) -> Result<(), anyhow::Error>{
        // Verify string is 2 chars
        let input_string = input_string.trim().trim_ascii();
        if input_string.to_ascii_lowercase().len() != 2{
            return Err(Error::msg("Input is not in correct format: 'xy'"));
        }


        // Split into x/y string

        let (x,y): (&str, &str) = input_string.split_at(1);

        let parse_res = (x.parse::<usize>(), y.parse::<usize>());
        match parse_res{
            (Ok(x_int), Ok(y_int)) => {
                self.cells[y_int][x_int] = CellState::CircleTaken;
            },
            _ =>{
                let msg = format!("Failed to resolve input. \n\t x: {} -> {:#?} \nt y: {} -> {:#?}", x, parse_res.0, y, parse_res.1);
                return Err(Error::msg(msg));
            }
        }

        Ok(())
    }
}


fn main() {
   let mut board = Board::new();

    board.print_board();


    loop{
        board.clear_board();
        board.print_board();
        if board.did_win(){
         
            break;
        }
      

        loop {
            println!("\n\nPlayer X! \n \t Enter a new Position using 'xy' format. Example: 00 -> 0,0 on the board. \n ");
            let mut input: String = "".to_string();
            let res = std::io::stdin().read_line(&mut input);
            
            
            match board.parse_input(&input){
                Ok(_) => {
                    break;
                },
                Err(err) => {
                    println!("Error: {}", err);
                },
            }

        }

       
    }
}
