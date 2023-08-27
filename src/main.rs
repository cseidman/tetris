use std::io::{self, BufRead};
use crate::tetromino::{Tetromino, TetroShape};

mod grid;
mod tetromino;

fn main() {

    let stdin = io::stdin();
    let input = stdin.lock();
    let buffered_input = io::BufReader::new(input);

    for line in buffered_input.lines() {
        let mut grid = grid::Grid::new();
        let shape_sequence = line.unwrap();
        //println!("shape_sequence: {}", shape_sequence.clone()) ;
        for piece in shape_sequence.split(',') {
            let (p, x) = piece.split_at(1);
            let x = x.parse::<usize>().unwrap();
            let shape = TetroShape::from(p);
            grid.place_tetromino(Tetromino::new(shape), x);

        }
        //println!("{}",grid) ;
        println!("{}",grid.get_highest_row());

    }
}