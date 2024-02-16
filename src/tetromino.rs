use crate::grid::Coord;
const SHAPE_WIDTH:usize = 4 ;
const SHAPE_HEIGHT:usize = 3 ;

pub(crate) enum TetroShape {
    Q,
    Z,
    S,
    T,
    I,
    L,
    J
}

impl Into<TetroShape> for &str {
    fn into(self) -> TetroShape {
        match self {
            "Q" => TetroShape::Q,
            "Z" => TetroShape::Z,
            "S" => TetroShape::S,
            "T" => TetroShape::T,
            "I" => TetroShape::I,
            "L" => TetroShape::L,
            "J" => TetroShape::J,
            _ => panic!("Unknown shape: {}", self),
        }
    }
}

/// Returns a new tetromino based on the `TetroShape` passed in.
/// The shape is defined as a 3x4 array of booleans, where true means the cell is filled
/// and false means the cell is empty. The rows of the array are the rows of the tetromino
/// in reverse order, so the first row of the array is the bottom row of the tetromino.
/// Example:
/// ```rust
/// let t:TetroShape = TetroShape::Q.into() ;
/// ```
impl Into<Tetromino> for TetroShape {
    fn into(self) -> Tetromino {
        match self {
            TetroShape::Q => Tetromino::q_shape(),
            TetroShape::Z => Tetromino::z_shape(),
            TetroShape::S => Tetromino::s_shape(),
            TetroShape::T => Tetromino::t_shape(),
            TetroShape::I => Tetromino::i_shape(),
            TetroShape::L => Tetromino::l_shape(),
            TetroShape::J => Tetromino::j_shape(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Tetromino {
    shape: [[bool; SHAPE_WIDTH]; SHAPE_HEIGHT],
}

impl Tetromino {

    fn q_shape() -> Self {
        Self {
            shape: [
                [true, true, false, false],
                [true, true, false, false],
                [false, false, false, false],
            ]
        }
    }
    fn s_shape() -> Self {
        Self {
            shape: [
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ]
        }
    }
    fn z_shape() -> Self {
        Self {
            shape: [
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ]
        }
    }
    fn t_shape() -> Self {
        Self {
            shape: [
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ]
        }
    }
    fn i_shape() -> Self {
        Self {
            shape: [
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ]
        }
    }
    fn l_shape() -> Self {
        Self {
            shape: [
                [true, true, false, false],
                [true, false, false, false],
                [true, false, false, false],
            ]
        }
    }
    fn j_shape() -> Self {
        Self {
            shape: [
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ]
        }
    }
    /// Returns a vector of filled coordinates for the tetromino relative to the coordinates passed in
    /// The coordinates passed in are the bottom left corner of the tetromino on the grid, but the first
    /// element of the defined shape.
    pub(crate) fn get_relative_location(&self, coordinates: Coord) -> Vec<Coord> {
        let mut coords = Vec::with_capacity(SHAPE_HEIGHT * SHAPE_WIDTH ) ;
        let mut row_num = 0 ;
        for row in self.shape {
            for (col, cell) in row.iter().enumerate() {
                if *cell {
                    let coord = Coord::new(coordinates.get_row()- row_num, col + coordinates.get_col()) ;
                    coords.push(coord) ;
                }
            }
            row_num += 1 ;
        }
        coords
    }

}