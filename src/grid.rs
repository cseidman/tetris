use std::fmt::{Display, Formatter};
use crate::tetromino::{Tetromino};
const HEIGHT:usize = 100 ;
const WIDTH:usize = 10 ;

/// This struct represents a coordinate on the grid or a tetromino
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

    /// Returns the highest row used on the grid from the bottom up
    pub(crate) fn get_highest_row(&self) -> usize {
        HEIGHT-self.first_used_row
    }

    /// Returns true if the tetromino can be placed at the specified coordinate without colliding
    /// with any other tetrominos on the grid.
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
    /// Places a tetromino on the grid at the specified column
    pub(crate) fn place_tetromino(&mut self, tetromino: Tetromino, column: usize) {

        // Get back the lowest row where the tetromino can be placed
        let lowest_row = self.find_lowest_row(tetromino, column) ;
        // Get the coordinates of the tetromino relative to the lowest row
        let locations = tetromino.get_relative_location(Coord::new(lowest_row, column)) ;

        for location in locations {
            // This is how we update the first_used_row
            if location.row < self.first_used_row {
                self.first_used_row = location.row ;
            }
            // We set the cell in the grid to true to indicate it's filled
            self.grid[location.row][location.col] = true ;
        }
        // We check for full rows and clear them
        self.clear_rows() ;
    }
    /// Clears any full rows and moves everything down
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
/// Displays the bottom 10 rows of the grid.
/// This comes in handy for debugging.
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
        assert_eq!(3, grid.get_highest_row()) ;
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
