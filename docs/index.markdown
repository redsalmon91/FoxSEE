A strong UCI chess engine written in Rust!

**Current Version** 2.9.3

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)

## Features

- Negamax Search with Alpha-Beta Pruning
- Quiescence Search
- Iterative Deepening
- Bitboards
- Zobrist Hashing
- Transposition Table
- Static Exchange Evalution (SEE)
- MVV/LVA
- Late Move Reduction
- Check Extension
- History Heuristic
- Killer Heuristic
- Aspiration Window
- Linear Evaluation
- Null-Move Pruning
- Perft

## #0 Test Position

`2k2r2/pp2br2/1np1p2q/2NpP2p/2PP2p1/1P1N4/P3Q1PP/3R1R1K b - - 8 27`

Time taken to find the best move (under the same hardware conditions):

- **v2.x** - 50ms
- **v1.x** - 400ms
- **v0.x** - 1500ms

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

_____
Copyright (C) 2020 Zixiao Han. All Rights Reserved.
