# Simplified Tetris Engine in Rust
**Claude Seidman**   
claude@vectorline.com  
August 28, 2023

## Description

This project provides a simplified Tetris engine written in Rust. The engine simulates a grid where Tetris pieces (represented by letters) enter from the top and come to rest at the bottom, following simplified rules based on the classic Tetris game. The program processes an input file that specifies sequences of Tetris pieces and their starting positions, then outputs the height of the resulting block configurations.

The pieces are represented as follows:

- **Q**: Square (2x2)
- **Z**: Z-shaped
- **S**: S-shaped
- **T**: T-shaped
- **I**: I-shaped (1x4)
- **J**: J-shaped
- **L**: L-shaped

Rotation of the shapes is not supported in this simplified model.  

## Features

- Simulates Tetris grid and falling pieces
- Supports 7 unique Tetris pieces
- Clears filled rows
- Outputs resulting grid heights to STDOUT
- Input is read from STDIN, which allows for easy file input redirection
- Efficient algorithms for piece placement and row clearing

## Requirements

- The latest version of Rust (no lower than 1.50.0)
- Cargo package manager which comes with the Rust toolchain

## Downloading and Extracting the Source Code

The source code for this project is available as a ZIP file. Follow these steps to extract it:

### For Windows Users:

1. Copy the ZIP file of the source code to a local directory.
2. Navigate to the folder where the ZIP file is located.
3. Right-click on the ZIP file and select "Extract All...".
4. Choose a destination folder where you want the extracted files to be saved.
5. Click on "Extract".

### For macOS Users:

1. Copy the ZIP file of the source code to a local directory.
2. Navigate to the folder where the ZIP file is located.
3. Double-click on the ZIP file, and macOS will automatically extract it into the same folder.

### For Linux Users:

1. Copy the ZIP file of the source code to a local directory.
2. Open a terminal and navigate to the folder where the ZIP file is located.
3. Run the following command to extract the ZIP file:

```bash
unzip tetris_claude_seidman.zip 
```

Once you've extracted the ZIP file, navigate to the `tetris` folder. You can then proceed to compile and run the project as described in the following sections of this README.

## Project Structure

The project is divided into three main files:

### `main.rs`

This is the entry point for the program. It initializes the game grid and controls the game loop. It's responsible for reading the input file, delegating the placement of tetrominos, and coordinating the game states.

### `grid.rs`

This file contains the `Grid` struct, which is responsible for managing the state of the game grid. It provides methods to place tetrominos, check for completed lines, and update the grid accordingly.

### `tetromino.rs`

This file defines the `Tetromino` struct and the associated `TetroShape` enum. These are used to represent the various shapes of tetrominos that can be placed on the grid. The `Tetromino` struct provides methods to create shapes according their type and to calculate relative positions on the grid. 



## Usage

The program reads from STDIN and writes to STDOUT, making it simple to use with input and output file redirection.

### Compile the Program

Compile the program using Cargo:

```bash
cargo build --release
```

### Run the Program

Run the program using input redirection to read from an input file and output redirection to write to an output file:

```bash
./target/release/tetris < input.txt > output.txt
```

Also, you can run the program without output redirection to see the resulting heights in the terminal:

```bash
./target/release/tetris < input.txt
```

You may also compile and run the program in one step:

```bash
cargo run --release < input.txt > output.txt
``` 

### Example

Given an input file (`input.txt`) with the following content:

```
I0,I4,Q8
T1,Z3,I4
Q0,I2,I6,I0,I6,I6,Q2,Q4
```

Running the program will produce an output like:

```
1
4
3
```

## Unit Tests

This project includes a series of unit tests to ensure the Tetris engine's accuracy and efficiency. The unit tests are contained in a the `grid` module, in keeping with Rust's idiomatic structure for unit tests.

### Displaying the Grid

The `Grid` struct implements the `Display` trait, which means you can easily visualize the grid state in the console by printing the `Grid` object. This is especially useful for debugging and for understanding the current grid state while running tests.
If you choose to run the tests with the `--nocapture` flag, you will see the grid state for each test.

```rust
println!("{}", grid);
```

### Test Functions

Here is a brief explanation of what each test function does:

- `ex1()`: Tests the placing of an `I` tetromino at position 0, another `I` at position 4, and a `Q` at position 8. Checks if the highest row occupied is 1.

- `ex2()`: Places a `T` tetromino at position 1, a `Z` at position 3, and an `I` at position 4. Verifies that the highest row occupied is 4.

- `ex3()`: Similar to the above tests but involves multiple placements and expects the highest row to be 3.

- `ex4()`: An advanced test that places various tetrominos and verifies that the highest row occupied is 5.

- `ex5()`: Tests various `L` and `J` shapes and expects the highest row to be 3.

- `get_location()`: Tests that the function returnrs the correct coordinates generated for each type of tetromino shape.

### Running Tests

To run the tests, navigate to the project directory and run:

```bash
cargo test
```

If you would like to see the grid state for each test, run:

```bash
cargo test -- --nocapture
```  

This will compile your project and run all the defined tests, outputting the results to the terminal so you can see which passed or failed.

