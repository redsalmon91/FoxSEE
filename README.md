# 🦊 FoxSEE
Strong UCI chess engine written in Rust!

**Current Version** 2.8.8

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)

## Features

**Since 0.x**
- 0x88 Board Representation
- Minimax Search with Alpha-Beta Pruning
- Quiescence Search
- Iterative Deepening
- Check Extension
- History Heuristic
- MVV/LVA

**Since 1.x**
- Zobrist Hashing
- Bitboards

**Since 2.x**
- Transposition Table
- Late Move Reduction
- Killer Heuristic
- Aspiration Window
- Linear Evaluation
- Null-Move Pruning
- Perft
- Negamax

**Deprecated**
- Static Exchange Evalution (SEE)
- Futility Pruning
- Passed-Pawn Extension

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
