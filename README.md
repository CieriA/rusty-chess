# Chess with Rust
A Chess implementation on the terminal (pass-to-play), written in [Rust](https://www.rust-lang.org).

## Features
- Grid display
- 2 players
- I/O to play the game
- Every piece of Chess
- Double move for the pawn the first time
- Castle
- En passant

## Features to implement (Coming soon!)
- Check
- Checkmate
- Stalemate

## Requirements
- **Rust** (stable) — install via [rustup](https://rustup.rs)

## Building the project
Clone the repository and build it in release mode:
```bash
git clone https://github.com/CieriA/rusty-chess
cd rusty-chess-test
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
- colored
- indexmap

### Docs
To build the documentation locally:
```bash
cargo doc --open
```

## License
This project is licensed under the ISC License. For more info see the [LICENSE](LICENSE) file.
