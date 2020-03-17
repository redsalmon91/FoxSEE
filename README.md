# FoxSEE
A strong UCI chess engine written in Rust!

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)
![Release Version](https://img.shields.io/github/v/release/redsalmon91/FoxSEE?color=orange)
![License](https://img.shields.io/github/license/redsalmon91/FoxSEE)

## Features

- Principal Variance Search
- Negamax Search
- Alpha-Beta Pruning
- Quiescence Search
- Iterative Deepening
- Aspiration Window
- Bitboards
- Zobrist Hashing
- Transposition Table
- Static Exchange Evalution (SEE)
- MVV/LVA
- Check Extension
- Mate-Threat Extension
- Relative History Heuristic
- Killer Heuristic
- Linear Evaluation
- Null-Move Pruning
- Late-Move Reductions
- Futility Pruning
- Delta Pruning
- Perft

## Position 0 Benchmark
```
2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27
```

On `i5-8250U`, below are the time needed to find the best move (by version):

- 0.x - 1500ms
- 1.x - 500ms
- 2.x - 50ms
- 3.x - 25ms

## How to build
Install [Rust](https://www.rust-lang.org/)

Run `cargo build --release`

## How to use
This program complies with the [UCI protocol](http://wbec-ridderkerk.nl/html/UCIProtocol.html), you can use it with any of the UCI-compatible GUIs (with a few limitations as mentioned in the [Limitations](#limitations) section).  
Aside from the standard set of UCI commands, `perft x` is also supported.

## Limitations
- Search `x` nodes is currently not supported
- Search `mate` in in `x` moves is currently not supported
- Search specific moves under a given position is currently not supported
- `ponder` is currently not supported
- Big-Endian systems are not supported

## References
[Chess Programming by François Dominic Laramée](http://archive.gamedev.net/archive/reference/articles/article1014.html)  
[Mediocre Chess Guides](http://mediocrechess.sourceforge.net/guides.html)  
[Chess Programming Wiki](https://www.chessprogramming.org)
