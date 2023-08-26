#![allow(dead_code)]
const WIDTH: usize = 10;
const HEIGHT: usize = 100;

#[derive(Copy, Clone)]
struct Location(usize, usize) ;
impl Location {
    fn row(&self) -> usize {
        self.0
    }
    fn column(&self) -> usize {
        self.1
    }
    fn move_up(&mut self) {
        self.0-=1;
    }
}

#[derive(Copy, Clone)]
struct Cell {
    location: Location,
    filled: bool 
}

impl Cell {
    fn new(location: Location) -> Self{
        Self {
            location,
            filled: false
        }
    }
}

const SHAPE_WIDTH:usize = 4 ;
const SHAPE_HEIGHT:usize = 3 ;
const BOTTOM_LEFT: usize = 8;

struct Tetromino {
    shape: [Cell;SHAPE_WIDTH*SHAPE_HEIGHT],
}

impl Tetromino {
    fn new(shape: [Cell;SHAPE_WIDTH*SHAPE_HEIGHT]) -> Self {
        Self {
            shape
        }
    }

    fn move_up(&mut self) {
        self.set_location(Location(self.shape[BOTTOM_LEFT].location.row()-1, self.shape[BOTTOM_LEFT].location.column()))   ;
    }

    fn move_down(&mut self) {
        if self.shape[BOTTOM_LEFT].location.row() == 0 {
            return ;
        }
        self.set_location(Location(self.shape[BOTTOM_LEFT].location.row()+1, self.shape[BOTTOM_LEFT].location.column()))   ;
    }

    fn set_location(&mut self, location: Location) {
        // Set the location of the shape at the bottom left of the shape
        self.shape[BOTTOM_LEFT].location = location ;
        // Set the location of the other cells
        for line in 0..SHAPE_HEIGHT {
            for col in 0..SHAPE_WIDTH {
                let mut cell = self.shape[line*SHAPE_WIDTH+col] ;
                if cell.filled {
                    cell.location = Location(location.row()+line, location.column()+col) ;
                }
            }
        }
    }

}

type GridLine = [Cell;WIDTH] ;

struct Canvas {
    grid: [GridLine; HEIGHT],
    first_used_line: Option<usize>,
    max_height: usize,
}

impl Canvas {
    fn new() -> Self {
        let mut grid = [[Cell::new(Location(0,0));WIDTH];HEIGHT] ;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                grid[i][j].location = Location(i,j) ;
            }
        }
        Self {
            grid,
            first_used_line: None,
            max_height: 0
        }
    }

    fn put_shape(&mut self, tetromino: Tetromino, column: usize) {
        // Set the start line right above the first used line
        let start_line = self.max_height-1 ;
        // Extract the shape from the tetromino
        let mut shape = tetromino.shape ;
        // Set the location of the shape at the bottom left of the shape
        shape[8].location = Location(start_line, column);

        // Move the shape up until it can be placed
        while !self.can_place_shape(&shape) {
            for line in 0..SHAPE_HEIGHT {
                for col in 0..SHAPE_WIDTH {
                    let mut cell = shape[line*SHAPE_WIDTH+col] ;
                    if cell.filled {
                        cell.location.move_up() ;
                    }
                }
            }
        }
        // Place the shape
        self.place_shape(&shape) ;

    }

    fn place_shape(&mut self, shape : &[Cell;SHAPE_WIDTH*SHAPE_HEIGHT]) {
        // Place the shape on the grid
        for i in 0..SHAPE_HEIGHT {
            for j in 0..SHAPE_WIDTH {
                let cell = shape[i*SHAPE_WIDTH+j] ;
                if cell.filled {
                    let row = cell.location.row() ;
                    let column = cell.location.column() ;
                    self.grid[row][column].filled = true ;
                }
            }
        }
    }

    fn can_place_shape(&self, shape: &[Cell;SHAPE_WIDTH*SHAPE_HEIGHT]) -> bool {
        // Check if the shape can be placed at the given location
        for i in 0..SHAPE_HEIGHT {
            for j in 0..SHAPE_WIDTH {
                let cell = shape[i*SHAPE_WIDTH+j] ;
                if cell.filled {
                    let row = cell.location.row() ;
                    let column = cell.location.column() ;
                    if row >= HEIGHT || column >= WIDTH {
                        return false ;
                    }
                    if self.grid[row][column].filled {
                        return false ;
                    }
                }
            }
        }
        true
    }

}   

fn main() {
    let canvas = Canvas::new() ;
    for j in 0..5 {
        for i in 0..8 {
            println!("{:?},{:?}", canvas.grid[j][i].location.0, canvas.grid[j][i].location.1);
        }
    }

}

#[cfg(test)]
mod test {
    #[test]
    fn place_terominos() {
        let mut canvas = super::Canvas::new() ;
        let shape = [
            super::Cell::new(super::Location(0,0)),
            super::Cell::new(super::Location(0,1)),
            super::Cell::new(super::Location(0,2)),
            super::Cell::new(super::Location(0,3)),

            super::Cell::new(super::Location(1,0)),
            super::Cell::new(super::Location(1,1)),
            super::Cell::new(super::Location(1,2)),
            super::Cell::new(super::Location(1,3)),

            super::Cell::new(super::Location(2,0)).filled = true,
            super::Cell::new(super::Location(2,1)).filled = true,
            super::Cell::new(super::Location(2,2)).filled = true,
            super::Cell::new(super::Location(2,3)).filled = true,
        ] ;
        canvas.place_shape(&shape) ;
        for j in 0..3 {
            for i in 0..4 {
                assert!(canvas.grid[j][i].filled) ;
            }
        }
    }
}