use std::io::{self, BufRead};
use crate::tetromino::{TetroShape};

mod grid;
mod tetromino;

fn main() {

    let buffered_input = io::BufReader::new(io::stdin());

    for line in buffered_input.lines() {
        let shape_sequence = line.unwrap();

        let mut grid = grid::Grid::new();

        shape_sequence.split(',').for_each(|piece| {
            let (p, x) = piece.split_at(1);
            let x = x.parse::<usize>().unwrap();
            let tetroshape: TetroShape = p.into() ;
            grid.place_tetromino(tetroshape.into(), x);
        }) ;

        println!("{}",grid.get_highest_row());
    }
}