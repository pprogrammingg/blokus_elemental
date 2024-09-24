# Desired Features
Designing a turn-based multiplayer game like Blokus in Rust, means we have to think on a highlevel what design should be followed, 
what Rust libraries are fast, supported, robust, and efficient with minimum amount of maintenance and gymnastics.

Table of Content
- [Messaging](./1_2_messaging.md): how  back-end components like player-to-server communications handled inside the backend and betwene back-end and the UI
- Game flow management: game flow determines when game has been created, started, being_played and ended
- leaderboard to store user stats
- Game State Persistence: how do we store the game state during play and in case of interruptions?
- Synchronization across clients: making sure all players have a synchronized view of the game and de-synchronization does not happen
- 
- Validation and game rules enforcement: making sure logic is validated on the back-end and so that cheating can not happen
- User auth and session keys: ability to enable login and session keys
- Testing and Simulation: Using AI and bots to do stress testing. I prefer real people testing it sooner than later to fix glitches and 
imbalances.
- Scalability Considerations: if the game scales up, can we handle mutliple games and session on the server? is there a limit? Multi-sharding 
the lobby? 
