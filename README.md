# FoxSEE Chess Engine 🦊
A tiny yet strong chess engine written in Rust!  

- Only 3000 lines of code (excluding tests)
- Very small memory footprint (less than 3MB)

**Current Version**  
1.0.1

[![Build Status](https://travis-ci.com/redsalmon91/FoxSEE.svg?branch=master)](https://travis-ci.com/redsalmon91/FoxSEE)

## Main Features

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
- UCI-compatible

## Other Features

- Extend search depth when in-check, limit to `extension_count * 2 <= ply`.
- Reduce search depth for non-cap moves, test with zero window, and re-search if it fails high.
- Re-captures are priorited
- `root_search` tracks history score from previous iteration to achieve better move ordering

## How to build
Run `cargo build --release`
