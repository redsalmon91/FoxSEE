# FoxSEE
A strong UCI chess engine written in Rust!

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)
![Release Version](https://img.shields.io/github/v/release/redsalmon91/FoxSEE?color=orange)
![License](https://img.shields.io/github/license/redsalmon91/FoxSEE)

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
- Delta Pruning
- Perft

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
