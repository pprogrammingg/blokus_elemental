Steps:
Connect Players:

Each player will connect via the WebSocket session, and once all 4 players are connected, the "Start Game" button becomes active.
You can use a server-side counter or state to track the number of connected players.
Start Game:

Clicking the "Start Game" button will send a message to the server, which broadcasts a "start" message to all connected clients.
The server can initialize the game state (empty 8x8 board) and broadcast the board state to all clients.
The server will also need to assign a turn order and tell the first player that it's their turn.
Player Turns:

Only the active player will be able to make a move. The server can manage this by sending a "turn" message to the specific player who should take the next move.
This message will include data like the current board state and available pieces for that player.
Playing a Turn:

The active player selects a piece and makes a move, sending this information to the server.
The server validates the move, updates the game state, and broadcasts the new board state to all clients.
Update Board and Next Turn:

After updating the board, the server assigns the turn to the next player and repeats the process until the game is over.
Possible Message Types for WS Communication:
Join Message: Sent when a client connects to the server.
Start Game Message: Broadcasted by the server once all players join and the game begins.
Turn Message: Sent by the server to a specific client when it’s their turn, containing the board state and available pieces.
Move Message: Sent by the active player to the server when they make a move.
Board Update Message: Broadcasted by the server after a valid move, containing the updated board state.
Game Over Message: Broadcasted when a win or draw condition is met.
Challenges to Address:
Concurrency: You need to make sure only one player can move at a time.
Validation: Moves need to be validated by the server to ensure they are legal.
Client Sync: Make sure all clients are synchronized with the current game state.
This structure will allow for a smooth, turn-based experience where the server controls turn order, board updates, and the game state while the clients handle player input and display.