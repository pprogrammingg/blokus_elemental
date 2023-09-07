const DEFAULT_BOARD_WIDTH: usize = 20;
const DEFAULT_BOARD_HEIGHT: usize = 20;

pub struct Board {
    width: usize,
    height: usize,
    grid:
     [[u8; DEFAULT_BOARD_WIDTH]; DEFAULT_BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Board {
            width: DEFAULT_BOARD_WIDTH,
            height: DEFAULT_BOARD_HEIGHT,
            grid: [[0; DEFAULT_BOARD_WIDTH]; DEFAULT_BOARD_HEIGHT],
        }
    }

    pub fn display(&self) {
        // Loop through each row and column to display the tiles
        for row in 0..self.height {
            for col in 0..self.width {
                let tile_state = self.grid[row][col];
                let tile_symbol = match tile_state {
                    0 => 'W', // Empty tile
                    1 => 'R', // Red tile
                    2 => 'G', // Green tile
                    3 => 'B', // Blue tile
                    4 => 'Y', // Yellow tile
                    _ => '?', // Unknown tile state (handle this appropriately)
                };
                print!("{} ", tile_symbol);
            }
            println!(); // Move to the next row after printing a full row of tiles
        }
    }

    pub fn set_tile_value(self: &mut Self, i: usize, j: usize, value: u8) -> &mut Self{
            self.grid[i][j] = value;
            self
    }
}
