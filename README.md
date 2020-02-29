# 🦊 FoxSEE
Strong UCI chess engine written in Rust!

**Current Version** 2.8.7

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
- Passed-Pawn Extension

**Deprecated**
- Static Exchange Evalution (SEE)
- Futility Pruning

## How to build
Install [Rust](https://www.rust-lang.org/)

Run `cargo build --release`

## How to use
This program complies with the [UCI protocol](http://wbec-ridderkerk.nl/html/UCIProtocol.html), you can use it with any of the UCI-compatible GUIs (with a few limitations as mentioned in the [Limitations](#limitations) section).  
Aside from the standard set of UCI commands, `perft x` is also supported.

## Limitations
- Infinite analysis mode is currently unsupported
- Search to depth is currently unsupported
- Big-endian systems are not supported
