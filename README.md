# w4-snake

A game written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

## Building

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

## Make

```text
❯ make help
 _          _       
| |__   ___| |_ __  
| '_ \ / _ \ | '_ \ 
| | | |  __/ | |_) |
|_| |_|\___|_| .__/ 
             |_|    
help                 💬 This help message :)
lint                 🔎 Check for linting and formatting errors
lint-fix             🧙 Fix linting and formatting errors
install-tools        🔮 Install dev tools and pre-reqs
build                🔨 Build the game cart WASM
clean                🧹 Clean up build artifacts
run                  🚀 Run the game and start the web server
watch                👀 Run the game with reload on file change
publish              🎁 Bundle for distribution (exe and HTML)
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
