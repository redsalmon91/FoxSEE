# 🦊 FoxSEE Chess Engine
UCI chess engine written in Rust!  

**Current Version** 2.8.5

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
- Null Move Pruning
- Perft
- Negamax

## How to build
Install [Rust](https://www.rust-lang.org/)

Run `cargo build --release`
