# Chess with Rust
A Chess implementation on the terminal (pass-to-play), written in [Rust](https://www.rust-lang.org).

## Features
- Grid display on the terminal
- 2 players
- I/O to play the game
- Every piece of Chess
- Double pawn push
- Castle
- En passant
- Pawn upgrade
- Check, Checkmate and Stalemate
- Tie by 50 moves rules
- Every normal feature of chess not cited in the [future features](#features-to-implement-coming-soon)

## Features to implement (Coming soon!)
- Tie by repeated moves (3 or 5)
- Tie by missing material (Now it only works when kings are the only piece on the board)
- Minimax to implement a bot

## Requirements
- **Rust** (stable) — install via [rustup](https://rustup.rs) (all OSs)
- **sdl3** — install via `brew install sdl3` (on macOS)
- **sdl3_image** — install via `brew install sdl3_image` (on macOS)

## Building the project
Clone the repository and build it in release mode:
```bash
git clone https://github.com/CieriA/rusty-chess
cd rusty-chess
cargo build --release
```

## Running the game
```bash
cargo run --release
```

## Controls
- A1-H8 / a1-h8 -> to index the board from the terminal

## Development notes
This project uses the following crates:
- colored _(planning to remove this)_
- indexmap
- sdl3

### Project Structure
- [`geomath`](src/geomath) module: utilities for indexing a matrix and enum
  representing a [`Direction`](src/geomath/rotation.rs).
- [`types`](src/types) module: contains each type used in this project
  (e.g. [`Piece`](src/types/piece), [`Movement`](src/types/movement.rs) struct).
- [`chessboard`](src/chessboard) module: struct `Board` to hold the state of the
  game.
- [`game`](src/game) module: **DEPRECATION PLANNED** struct `Game` to hold the
  logic of the game and the starting function (working on the terminal).
- [`interface`](src/interface) module: struct `Interface` that substitute
  `Game` to play the game on a GUI instead of the terminal.

### Docs
To build the documentation locally:
```bash
cargo doc --open
```

## License
This project is licensed under the ISC License. For more info see the [LICENSE](LICENSE) file.
