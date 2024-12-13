use std::{cmp::min, io::Read, sync::{Arc, Mutex}, thread, time::Duration};
use crossterm::event::{self, Event, KeyCode};
const PX_EMPTY: u8 = 0;

pub trait Drawable: Send {
    fn get_sprite(&self) -> [[u8; 4]; 4];
}

#[derive(Default)]
pub struct FourCube;

impl Drawable for FourCube {
    fn get_sprite(&self) -> [[u8; 4]; 4] {
        [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0]]
    }
}

#[derive(Default)]
pub struct FourBar;

impl Drawable for FourBar {
    fn get_sprite(&self) -> [[u8; 4]; 4] {
        [[1, 0, 0, 0], 
        [1, 0, 0, 0], 
        [1, 0, 0, 0], 
        [1, 0, 0, 0]]
    }
}


#[derive(Default)]
pub struct TriThing;

impl Drawable for TriThing {
    fn get_sprite(&self) -> [[u8; 4]; 4] {
        [[0, 0, 0, 0], 
        [0, 0, 0, 0], 
        [0, 1, 0, 0], 
        [1, 1, 0, 0]]
    }
}

pub struct Board {
    pub pixels: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    sprites: Vec<Box<dyn Drawable>>,
    sprite_locations: Vec<(usize, usize)>,
    active: Option<usize>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let board = Self::gen_empty_board(width, height);
        Self {
            width,
            height,
            pixels: board,
            sprites: vec![],
            sprite_locations: vec![],
            active: None
        }
    }

    fn gen_empty_board(width: usize, height: usize) -> Vec<Vec<u8>> {
        let mut board = Vec::with_capacity(height);


        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(PX_EMPTY);
            }
            board.push(row);
        }



        board
    }

    fn clear_board(&mut self ) {
        for y in 0..self.pixels.len() {
          
            for x in 0..self.pixels[y].len() {
                self.pixels[y][x] = PX_EMPTY;
            }
          
        }


    }

    pub fn new_sprite(&mut self, sprite: Box<dyn Drawable>) {
        let init_pos = (self.width / 2, 0);
        self.sprites.push(sprite);

        let idx = self.sprites.len() - 1;

        self.sprite_locations.insert(idx, init_pos);
        self.active = Some(idx);
    }
    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        return self.pixels[y][x];
    }

    pub fn input(&mut self, dir_right: bool) {
        if let Some(active_idx) = self.active{
            let pos = &mut self.sprite_locations[active_idx];
            pos.0 = pos.0 + match dir_right { true => 1, false => 0}
        }
    }
    fn move_sprite(&mut self) {
        let board = &self.pixels;
        for (idx, sprite_pos) in self.sprite_locations.iter_mut().enumerate() {
            let sprite = &mut self.sprites[idx];

            let our_key = u8::MAX - idx as u8;
            let (current_x, current_y) = *sprite_pos;
            
            let mut board: Vec<Vec<u8>> = board.clone();
            for y in 0..self.pixels.len(){
                for x in 0..self.pixels[y].len(){
                   if board[y][x] == our_key{
                        board[y][x] = 0;
                   }
                }
            }

            // Check the lowest row for pixels (x range)

            let mut move_valid = true;

            let pattern = sprite.get_sprite();
            let mut desired_y = min(self.height, current_y + 1);
            for pattern_y in 0..4 {
                for pattern_x in 0..4 {
                    let now_x = min(self.width -1, pattern_x + current_x);
                    let now_y = min(self.height -1, pattern_y + current_y);

                    let future_x = min(self.width -1, now_x);
                    let future_y = min(self.height-1 , now_y + 1);

                    let current_val = pattern[pattern_y][pattern_x];
                    let future_val = board[future_y][future_x];

                    if current_val != 0 && (future_val != 0 ) {
                       move_valid = false;
                       println!("COLLISION: {}.{} with {}", future_x, future_y, future_val);
                    }

                    if future_y == self.height {
                        move_valid = false;
                    }
                }
            }

            if move_valid {
                sprite_pos.1 = desired_y
            }
        }
    }

    fn draw_sprite(&mut self) {
        
        for (idx, sprite) in self.sprites.iter().enumerate() {
            let pattern = sprite.get_sprite();
            let our_key = u8::MAX - idx as u8;
            let (x, y) = self.sprite_locations[idx];

            // apply the pattern

            for pattern_y in 0..4 {
                for pattern_x in 0..4 {
                    let draw_x = min(self.width - 1, pattern_x + x);
                    let draw_y = min(self.height - 1, pattern_y + y);
                    self.pixels[draw_y][draw_x] = match pattern[pattern_y][pattern_x] {
                        1 => {our_key},
                        _ => {0},
                    };
                }
            }
        }
    }
    pub fn print(&mut self) {
        clearscreen::clear().expect("failed to clear screen");
        self.clear_board();
        self.draw_sprite();
        self.move_sprite();

        print!("||");
        for x in 0..self.width {
            print!("==");
        }
        print!("||");
        println!("|");
        for y in 0..self.pixels.len() {
            // Row drawing
            print!("||");
            for x in 0..self.pixels[y].len(){
                let val = self.pixels[y][x];
                let to_print = match val {
                    1 => "**",
                    PX_EMPTY => "  ",
                    _ => "[]",
                };
                print!("{}", to_print);
            }
            print!("||");
            println!("|");
        }

        print!("||");
        for x in 0..self.width {
            print!("==");
        }
        print!("||");
        println!("|");
    }
}

fn random_peice() -> Box<dyn Drawable>{

    let choice: u8 = rand::random();


    match choice{
        0..40 =>{ Box::new(FourCube::default())},
        100..200 =>{ Box::new(TriThing::default())}
        _ => { Box::new(FourBar::default())},
    }

}
fn main() {
    println!("Hello, world!");

    let board = Arc::new(Mutex::new( Board::new(10, 20)));

    thread::scope(|s| {
        let board_clone = board.clone();
        s.spawn(move || {
        
            loop {
                if event::poll(Duration::from_millis(1)).unwrap() {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        match key_event.code {
                            KeyCode::Char('a') => {
                                let mut board = board_clone.lock().unwrap();
                                board.input(false);
                                println!("MOVING LEFT");
                            }
                            KeyCode::Char('d') => {
                                let mut board = board_clone.lock().unwrap();
                                board.input(true);
                                println!("MOVING RIGHT");
                            }
                            _ => {}
                        }
                    }
                }
                
            }
        });
        let mut board = board.lock().unwrap();
        for t in 0..100 {
            board.print();

            if t % 10 == 0 {
                println!("Spawning new piece");
                board.new_sprite(random_peice());
            }

            std::thread::sleep(Duration::from_secs_f64(0.5));
        }
    });

}
