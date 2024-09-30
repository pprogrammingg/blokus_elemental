
# Blockchain And Web3
Blockchain can bring certain scalability advantages to the software at hand based on the following:

- Scalability: having a distributed storage system like blockchain is essentially like a write-only database.
Some blockcain are supporting massive throughuts and read/write speeds and hence they are suitable for modern game softwate.

- Availability: blockchains need to be up with minimum downtime to perserve their value proposition, crashes should be uncommon
and even lower than traditional Web2 counterparts like cloud databases

- Persistence and Synchronicity: Blockchain are a great way to have information synched between various involved parties. This 
is one of the huge value propositions of such technology.

## What are the advantages of Blockchain and Web3 for Blokus Elemental?
We can persist:

- Game state and Syncronicity: Persisting every move can be expensive, storage and gas heavy. Maybe at the begining, a final 
or interrupted version of game can be stored, so that eligible users can look at or resume playing it.
However, for desired game session, every move have a history of states stored so that one can replay the game. Ofcourse, this is 
much more gas-heavy.

    - Storing final or interrupted version of game: this can be a greatly desired feature and alleviates certain complexitieis of browser
    and session / authentication managment.

    For example, when a game is created, an NFT associated with that game can be minted and stored in the smart contract component. 
    Moves in the game update the NFT data and if game is finisehd or interrupted this NFT can remain there until a certain amount of time.
    (Maybe an cron job can look at all NFTs in the component and burn items older than a certain timestamp)

- Cheating and rule enforcement: it is possible to enforce game rules completely on smart contracts, however, such work would incur 
huge gas fees and computation and might also be slower than Web 2 coutnerpart in 2024.

However, maybe one nifty thing here is that a set of rules can be persisted in a smart contract and be gauranteed not to change 
via immutability of the blockchain. One cheap read at the start of the game, brings the rules read from the smart contract and 
in combination with the web2 backend logic performs all the actions necessary.

This way rules don't have to be stored in a centralized database subject to arbitrary updates... infact it seems so that Ethereum 
was created to solve this very issue - see  [NFT mastermind says he created Ethereum because Warcraft nerfed his character](https://www.polygon.com/22709126/ethereum-creator-world-of-warcraft-nerf-nft-vitalik-buterin).

An unexplored advantage of this is that, in a mature game, players can choose different ruleset versions of the game based on 
different smart contracts to connect to if they feel another version was more balanced. I remember in Starcraft every new update
had people complaining - "yo, lurker is imba imo" ~ ZergPants would comment.

    - Security consideration
    Interacting with smart contract and taking rulesets off there can pose a risk where peopel make their own rule versions, deploy a contract and ask others to 
    play the game with their own rules. This should be mitigates via concepts of like dApp definition accounts such as the one Radix implemented, essentially 
    a two way verification that Web3 parts of the dApp need to conform to certain contract addresses only. See [dApp Definition Account](https://docs.radixdlt.com/docs/dapp-definition-setup#:~:text=The%20role%20of%20the%20dApp,fake%20representation%20of%20your%20dApp) 


## What are the barriers and disadvantages of Blockchain
- Gas Fees: this seem to be a huge issue. I do not want to pay gas using a casual game specially when different versions of it exist for free (albeit with ads) online. 
In a game that gets traction and can create a community to desire and hold economic assets in the game, then this is not an issue as cash flow can sponsor the game. 

For now for a casual version, let's keep it on a test net where gas is free :)

- Ruleset and other computation heavy tasks : Certain aspects such as enforcing rules and heavy computations such as what moves and pieces are left for a player can be excpensive and slower considering the round-trip to/from blockchain. A back-end can process with scale and replicas if needed.

## Choice of Blockchain
Any blockchain and ideally the more blockchains integrated with the better as it allows more crypto tribes to participate.
For the initial phase, Radix DLT is chosen, as it exhibits one of the best throughputs and cheap gas fees without the friction of bridging to side-chains and L2s.
Some disadvantages here are the Raidx is relatively new and less users utilize it hence less network effect; however, with such innovative potential, like breaking
the 1M TPS barrier in testnet (See official Radix website).

Ideally, more popular blockchains L1s and L2s can be added, these can include Solana, AVAX, NEAR, SUI, etc.




