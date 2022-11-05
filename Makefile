SHELL := /bin/bash
BUILD = build
WASM_PATH = target/wasm32-unknown-unknown/release
OUT = dist
BIN = bin
TITLE = Snek Game

.PHONY: help install-tools build clean
.DEFAULT_GOAL = build

help: ## 💬 This help message :)
	@figlet $@ || true
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

lint:
	@figlet $@ || true
	@cargo fmt --all -- --check
	@cargo clippy

lint-fix:
	@figlet $@ || true
	@cargo fmt --all --
	@cargo clippy

install-tools: ## 🔮 Install dev tools and pre-reqs
	@figlet $@ || true
	@wget -q https://github.com/aduros/wasm4/releases/latest/download/w4-linux.zip
	@unzip ./w4-linux.zip -d ./bin/
	@rm ./w4-linux.zip

build: ## 🔨 Build the game cart WASM
	@figlet $@ || true
	cargo build --release

clean: ## 🧹 Clean up build artifacts
	@rm -rf $(BUILD)
	@rm -rf $(OUT)

run: ## 🚀 Run the game and start the web server
	@$(BIN)/w4 run $(WASM_PATH)/cart.wasm --no-qr

watch: ## 👀 Run the game with reload on file change
	@$(BIN)/w4 watch --no-qr

publish: build ## 🎁 Bundle for distribution (exe and HTML)
	@$(BIN)/w4 bundle $(WASM_PATH)/cart.wasm --html $(OUT)/index.html --title "$(TITLE)" --icon-file assets/icon.png
	@$(BIN)/w4 bundle $(WASM_PATH)/cart.wasm --linux $(OUT)/game --title "$(TITLE)"
	@$(BIN)/w4 bundle $(WASM_PATH)/cart.wasm --windows $(OUT)/game.exe --title "$(TITLE)"