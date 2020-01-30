# FoxSEE Chess Engine 🦊
A tiny yet strong UCI chess engine written in Rust!  

- Only 3000 lines of code (excluding tests)
- Very small memory footprint (around 3MB depending on the os arch)

**Current Version**  
1.1.0

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)

## Features

- 0x88 Board Representation
- Minimax Search with Alpha-Beta Pruning
- Aspiration Window
- History Heuristic
- Refutation Table
- Quiescence Search
- SEE
- MVV-LVA
- Piece-square-value Table
- Zobrist Hashing
- Partial bitboards with rook, pawn, and king surroundings
- Depth extension for checks
- Depth reduction & zero-window search for non-captures
- Prioritized re-captures
- Root search with memory to track scores from previous iterations

## How to build
Run `cargo build --release`
