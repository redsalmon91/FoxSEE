UCI chess engine written in Rust!

![Release Version](https://img.shields.io/github/v/release/redsalmon91/FoxSEE?color=green)
![License](https://img.shields.io/github/license/redsalmon91/FoxSEE)

## Latest Rankings
[CCRL Standard](https://computerchess.org.uk/ccrl/4040/)  
[CCRL Blitz](https://ccrl.chessdom.com/ccrl/404/)  
[BRUCE Bullet](https://e4e6.com/)

## Features

- Negamax Search with Alpha-Beta Pruning
- Principal Variance Search
- Quiescence Search
- Iterative Deepening
- Internal Iterative Deepening
- Kindergarten Bitboards
- Zobrist Hashing
- Transposition Table
- Static Exchange Evalution (SEE)
- Null-Move Pruning
- Delta Pruning
- Futility Pruning
- Time Pruning ([wiki](https://github.com/redsalmon91/FoxSEE/wiki/Time-Pruning))
- Multi-Cut Pruning
- Razoring
- Check Extensions
- Singular Extensions
- Mate-Threat Extensions
- Late-Move Reductions
- Killer Heuristic
- Relative History Heuristic
- Counter-Move Heuristic

## How to build
Install [Rust](https://www.rust-lang.org/learn/get-started)

Run `cargo build --release`

## How to use
This program complies with the [UCI protocol](http://wbec-ridderkerk.nl/html/UCIProtocol.html), you can use it with any of the UCI-compatible GUIs (with a few limitations as mentioned in the [Limitations](#limitations) section).  

## Limitations
- Search `x` nodes NOT not supported
- Search `mate` in `x` moves NOT not supported
- Search specific moves under a given position is NOT supported
- `ponder` is currently NOT supported
- Big-endian systems are NOT supported

## References
[Chess Programming by François Dominic Laramée](http://archive.gamedev.net/archive/reference/articles/article1014.html)  
[Mediocre Chess Guides](http://mediocrechess.sourceforge.net/guides.html)  
[Chess Programming Wiki](https://www.chessprogramming.org)  
[Xorshiro128**](http://prng.di.unimi.it/)  
[Stockfish Evaluation Guide](https://hxim.github.io/Stockfish-Evaluation-Guide/)
