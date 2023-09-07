use super::game::board;

type GridType = [[u8; DEFAULT_BOARD_WIDTH]; DEDEFAULT_BOARD_HEIGHT];
#[test]
fn test_game_board_set_tile_value() {
    let mut board = Board::new();

    let board = board.set_tile_value(0, 2, 1);
    println!("new after setting");
}

fn util_compare_grids(actualGrid: GridType, expectedGrid: GridType){
    let mut board = Board::new();
    let board = board.set_tile_value(0, 2, 1);

    for row in 0..DEFAULT_BOARD_HEIGHT {
        for col in 0..DEFAULT_BOARD_WIDTH {
            let value = board[row][col];
            if(row == 0 && col == 2){
                //assert value eq 1
            } else {
                //assert value eq 0
            }
        }
    }
}