// Bismi Allah AlRahman AlRaheem
// Goal is to please Him alone insha-Allah ameen.
// Fri, Sat and Sun Blokus Elemental Time

use blokus_elemental::game::board::Board;

fn main() {
    println!("hello world");

    let mut board = Board::new();
    board.display();

    // let board = board.set_tile_value(0, 2, 1);
    // println!("new after setting");
    // board.display();

    // handle game custom configurations

    // initiate board, call UI render

    // Start a loop to manage turn

    // while game_is_not_ended
    // choose the next valid player, allow UI to accept arrow key input for the player
    // and notify on screen the player's turn

    // PLAYER_TURN_OVER=false
    // CHOICE_MODAL=true and PIECE_COLLECTION(containing valid and invalid pieces) sent to UI
    // while PLAYER_TURN_OVER=false,
    // if Turn Timer task interrupts -> PLAYER_TURN_OVER=true
    // Insert/finalize random piece in an available location
    // Call UPDATE_TILE_VS_PIECE_MAP\
    // Display valid and invalid pieces to pick for the player -> call UI render
    // while PIECE_CHOSEN=false
    // if player chooses a piece
    // PIECE_CHOSEN=true
    // move piece to first valid location (in INSERT_FINAL=false mode) -> render UI showing all possible piece locations (take first option of rotation configs)
    // while PIECE_INSERTED=false listen for user input event
    // CANCEL_CHOICE_EVENT -> PIECE_INSERTED=false
    // MOVE_EVENT -> move to next valid location, along with AVAILABLE_ROTATION_OPTIONS -> update UI
    // ROTATION_EVENT -> update piece_config = next_rotation_config -> update UI
    // INSERT_EVENT (could be from Enter key or mouse-left)

    // while UNDO_TIMER_OVER=false, enable Undo button, listen for UNDO_INSERTION_EVENT or FINALIZE_INSERTION_EVENT --> render UI

    // FINALIZE_INSERTION_EVENT =>
    // Update UI -> UNDO_TIMER_OVER=true, PIECE_INSERTED=true
    // bg task updates Tile VS possible piece table

    // UNDO_EVENT =>
    // Update UI -> UNDO_TIMER_OVER=true, PIECE_CHOSEN=false

    // Else => do nothing
    // TURN Loop over
    // CURRENT_PLAYER=NEXT_VALID_TURN.next()
    // GAME LOOP ENDED
    // Calculate score and show ranking
}
