use std::fmt::{Display, Formatter};
use crate::tetromino::{Tetromino};
const HEIGHT:usize = 100 ;
const WIDTH:usize = 10 ;


#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Coord {row, col}
    }

    pub(crate) fn get_row(&self) -> usize {
        self.row
    }

    pub(crate) fn get_col(&self) -> usize {
        self.col
    }

}

pub(crate) struct Grid {
    pub(crate) grid: [[bool; WIDTH]; HEIGHT],
    first_used_row: usize,
}
impl Grid {
    pub(crate) fn new() -> Self {
        Grid {
            grid: [[false; WIDTH]; HEIGHT],
            first_used_row: HEIGHT-1
        }
    }

    pub(crate) fn get_highest_row(&self) -> usize {
        HEIGHT-self.first_used_row
    }

    fn can_place_tertomino(&self, teromino: Tetromino, coord: Coord) -> bool {

        if coord.row == HEIGHT {
            return false ;
        }
        let locations = teromino.get_relative_location(coord) ;

        for location in locations {
            if self.grid[location.row][location.col] {
                return false ;
            }
        }
        true
    }

    fn find_lowest_row(&mut self, tetronimo: Tetromino, column: usize) -> usize {
        // We set this to the row above the top of highest row used on the grid
        let mut coord = Coord::new(self.first_used_row-1, column) ;
        // We move the coordinates to a row below until we can't place the tertomino there
        while self.can_place_tertomino(tetronimo, Coord::new(coord.row+1, column)) {
            coord.row += 1 ;
        }
        // This is the row where the tetromino can be placed
        coord.row
    }

    pub(crate) fn place_tetromino(&mut self, tetromino: Tetromino, column: usize) {

        let lowest_row = self.find_lowest_row(tetromino, column) ;

        let locations = tetromino.get_relative_location(Coord::new(lowest_row, column)) ;
        for location in locations {
            if location.row < self.first_used_row {

                self.first_used_row = location.row ;
            }
            self.grid[location.row][location.col] = true ;
        }
        self.clear_rows() ;
    }

    fn clear_rows(&mut self) {

        for row in self.first_used_row..HEIGHT {
            // If the row is full, we move everything down
            if self.grid[row].iter().all(|&x| x) {
                for r in (1..row+1).rev() {
                    self.grid[r] = self.grid[r-1] ;
                }
                // As the program finds and eliminates full rows, it increments the first_used_row
                // to make it lower on the grid than it was.
                // If we've already hit bottom, we don't increment the first_used_row.
                if self.first_used_row < HEIGHT {
                    self.first_used_row += 1 ;
                }
            }
        }
    }
}

impl Display for Grid {
   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rownum in 90..HEIGHT {
            let row = &self.grid[rownum];
            write!(f, "{:2}: ", HEIGHT - rownum).expect("Error writing to formatter");
            for cell in row.iter() {
                if *cell {
                    write!(f,"X").expect("Error writing to formatter");
                } else {
                    write!(f,".").expect("Error writing to formatter");
                }
            }
            writeln!(f).expect("Error writing to formatter") ;
        }
       Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::* ;
    use crate::tetromino::{Tetromino, TetroShape};

    fn display_grid(grid: &Grid) {
        for rownum in 90 .. HEIGHT {
            let row = &grid.grid[rownum] ;
            print!("{:2}: " , HEIGHT-rownum) ;
            for cell in row.iter() {

                if *cell {
                    print!("X") ;
                } else {
                    print!(".") ;
                }
            }
            println!() ;
        }
    }

    #[test]
    fn ex1() {
        let mut grid = Grid::new();
        grid.place_tetromino(Tetromino::new(TetroShape::I), 0);
        grid.place_tetromino(Tetromino::new(TetroShape::I), 4);
        grid.place_tetromino(Tetromino::new(TetroShape::Q), 8);

        println!("{}", grid) ;
        assert_eq!(1, grid.get_highest_row());

    }

    #[test]
    fn ex2() {
        let mut grid = Grid::new() ;

        grid.place_tetromino(Tetromino::new(TetroShape::T), 1) ;
        grid.place_tetromino(Tetromino::new(TetroShape::Z), 3) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 4) ;

        println!("{}", grid) ;
        assert_eq!(4, grid.get_highest_row()) ;

    }
    #[test]
    fn ex3() {

        let mut grid = Grid::new() ;

        grid.place_tetromino(Tetromino::new(TetroShape::Q), 0) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 2) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 6) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 0) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 6) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 6) ;
        grid.place_tetromino(Tetromino::new(TetroShape::Q), 2) ;
        grid.place_tetromino(Tetromino::new(TetroShape::Q), 4) ;

        println!("{}", grid) ;
        assert_eq!(3, grid.get_highest_row()) ;
    }

    #[test]
    fn ex4() {

        let mut grid = Grid::new() ;

        grid.place_tetromino(Tetromino::new(TetroShape::Q), 0) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 2) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 6) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 0) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 6) ;
        grid.place_tetromino(Tetromino::new(TetroShape::I), 6) ;
        grid.place_tetromino(Tetromino::new(TetroShape::Q), 2) ;
        grid.place_tetromino(Tetromino::new(TetroShape::Q), 2) ;
        grid.place_tetromino(Tetromino::new(TetroShape::Q), 4) ;

        println!("{}", grid) ;
        assert_eq!(5, grid.get_highest_row()) ;
    }
    #[test]
    fn ex5() {
        let mut grid = Grid::new() ;

        grid.place_tetromino(Tetromino::new(TetroShape::L), 0) ;
        grid.place_tetromino(Tetromino::new(TetroShape::J), 3) ;
        grid.place_tetromino(Tetromino::new(TetroShape::L), 5) ;
        grid.place_tetromino(Tetromino::new(TetroShape::J), 8) ;
        grid.place_tetromino(Tetromino::new(TetroShape::T), 1) ;

        println!("{}", grid) ;
        //assert_eq!(5, grid.get_highest_row()) ;
    }

    #[test]
    fn get_location() {
        {
            let teromino = Tetromino::new(TetroShape::Q);
            let coords = teromino.get_relative_location(Coord::new(10, 2));
            assert_eq!(coords.len(), 4);
            assert_eq!(coords[0], Coord::new(10, 2));
            assert_eq!(coords[1], Coord::new(10, 3));
            assert_eq!(coords[2], Coord::new(9, 2));
            assert_eq!(coords[3], Coord::new(9, 3));
        }

        {
            let teromino = Tetromino::new(TetroShape::S);
            let coords = teromino.get_relative_location(Coord::new(10, 2));
            assert_eq!(coords.len(), 4);
            assert_eq!(coords[0], Coord::new(10, 2));
            assert_eq!(coords[1], Coord::new(10, 3));
            assert_eq!(coords[2], Coord::new(9, 3));
            assert_eq!(coords[3], Coord::new(9, 4));
        }

        {
            let teromino = Tetromino::new(TetroShape::Z);
            let coords = teromino.get_relative_location(Coord::new(10, 2));
            assert_eq!(coords.len(), 4);
            assert_eq!(coords[0], Coord::new(10, 3));
            assert_eq!(coords[1], Coord::new(10, 4));
            assert_eq!(coords[2], Coord::new(9, 2));
            assert_eq!(coords[3], Coord::new(9, 3));
        }

        {
            let teromino = Tetromino::new(TetroShape::T);
            let coords = teromino.get_relative_location(Coord::new(10, 2));
            assert_eq!(coords.len(), 4);
            assert_eq!(coords[0], Coord::new(10, 3));
            assert_eq!(coords[1], Coord::new(9, 2));
            assert_eq!(coords[2], Coord::new(9, 3));
            assert_eq!(coords[3], Coord::new(9, 4));
        }

        {
            let teromino = Tetromino::new(TetroShape::I);
            let coords = teromino.get_relative_location(Coord::new(10, 2));
            assert_eq!(coords.len(), 4);
            assert_eq!(coords[0], Coord::new(10, 2));
            assert_eq!(coords[1], Coord::new(10, 3));
            assert_eq!(coords[2], Coord::new(10, 4));
            assert_eq!(coords[3], Coord::new(10, 5));
        }

        {
            let teromino = Tetromino::new(TetroShape::L);
            let coords = teromino.get_relative_location(Coord::new(10, 2));
            assert_eq!(coords.len(), 4);
            assert_eq!(coords[0], Coord::new(10, 2));
            assert_eq!(coords[1], Coord::new(10, 3));
            assert_eq!(coords[2], Coord::new(9, 2));
            assert_eq!(coords[3], Coord::new(8, 2));
        }

    }
}
