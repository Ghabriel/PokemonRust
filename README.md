# Pokémon Rust
A Rust implementation of a Pokémon game.

# Project dependencies

This project currently requires the latest version of nightly Rust to compile, which can be obtained through `rustup`:

```
rustup install nightly
```

# Build instructions

To run the game in a debug build (less performant but faster to compile and easier to debug):

```
cargo +nightly run
```

To run it in a release build for maximum performance:

```
cargo +nightly run --release
```

# Documentation

The documentation covers the most important parts of the code and is a work in progress for the rest. To generate it, run:

```
cargo +nightly doc --no-deps --open
```

An online version of it can also be found [here](https://ghabriel.github.io/PokemonRust/pokemon_rust/).
