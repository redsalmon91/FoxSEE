A master-level UCI chess engine written in Rust!

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)
![Release Version](https://img.shields.io/github/v/release/redsalmon91/FoxSEE?color=orange)
![License](https://img.shields.io/github/license/redsalmon91/FoxSEE)

## Features

- Principal Variance Search
- Negamax Search with Alpha-Beta Pruning
- Quiescence Search
- Iterative Deepening
- Internal Iterative Deepening
- Aspiration Window
- Bitboards
- Zobrist Hashing
- Transposition Table
- Static Exchange Evalution (SEE)
- MVV/LVA
- Check Extensions
- Deep Search Extensions
- Passed-Pawn Extensions
- Singular Extensions
- Late-Move Reductions
- History Heuristic
- Killer Heuristic
- Tapered Evaluation
- Piece-Square Tables
- Null-Move Pruning
- Multi-Cut Pruning
- Mate Distance Pruning
- Delta Pruning
- Reversed Futility Pruning
- Perft

## How to build
Install [Rust](https://www.rust-lang.org/learn/get-started)

Run `cargo build --release`

## How to use
This program complies with the [UCI protocol](http://wbec-ridderkerk.nl/html/UCIProtocol.html), you can use it with any of the UCI-compatible GUIs (with a few limitations as mentioned in the [Limitations](#limitations) section).  
Aside from the standard set of UCI commands, `perft x` is also supported.

## Limitations
- Search `x` nodes is currently not supported
- Search `mate` in `x` moves is currently not supported
- Search specific moves under a given position is currently not supported
- `ponder` is currently not supported
- Big-Endian systems are not supported

## References
[Chess Programming by François Dominic Laramée](http://archive.gamedev.net/archive/reference/articles/article1014.html)  
[Mediocre Chess Guides](http://mediocrechess.sourceforge.net/guides.html)  
[Chess Programming Wiki](https://www.chessprogramming.org)  
[Xorshiro128**](http://prng.di.unimi.it/)  
[Stockfish Evaluation Guide](https://hxim.github.io/Stockfish-Evaluation-Guide/)
