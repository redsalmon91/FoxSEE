# 🦊 FoxSEE Chess Engine
UCI chess engine written in Rust!  

**Current Version** 2.1.2

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)

## Features

**Since 0.x**
- 0x88 Board
- Minimax Search with Alpha-Beta Pruning
- Quiescence Search
- Iterative Deepening
- Check Extension
- Static Exchange Evaluation
- History Heuristic
- MVV/LVA

**Since 1.x**
- Zobrist Hashing
- Bitboards

**Since 2.x**
- Transposition Table
- Late Move Reduction
- Killer Heuristic

## How to build
Install [Rust](https://www.rust-lang.org/)

Run `cargo build --release`
