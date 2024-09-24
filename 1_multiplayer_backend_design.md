# Desired Features
Designing a turn-based multiplayer game like Blokus in Rust, means we have to think on a highlevel what design should be followed, 
what Rust libraries are fast, supported, robust, and efficient with minimum amount of maintenance and gymnastics.

I can think of the following:
- Creation of a lobby for unjoined people to form a game
- Game creation and waiting for slots to fill and ability to just start
- A Turn Management component: determines whose turn it is and when turn is finished
- A messaging framework: send and receive messages from back-end to/from UI
- Determine when game ends 
- Determine what libraries to use for each
- leaderboard to store user stats

Doing some research, a bunch of great ideas get included:
- Game State Persistence: how do we store the game state during play and in case of interruptions?
- Synchronization across clients: making sure all players have a synchronized view of the game and de-synchronization does not happen
- Validation and game rules enforcement: making sure logic is validated on the back-end and so that cheating can not happen
- User auth and session keys: ability to enable login and session keys
- Testing and Simulation: Using AI and bots to do stress testing. I prefer real people testing it sooner than later to fix glitches and 
imbalances.
- Scalability Considerations: if the game scales up, can we handle mutliple games and session on the server? is there a limit? Multi-sharindg 
the lobby? 
