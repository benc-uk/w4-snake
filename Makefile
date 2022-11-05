# Build dependencies
GO = tinygo
WASM_OPT = wasm-opt
SHELL := /bin/bash
BUILD = build
OUT = dist
BIN = bin
DEBUG = 0
REPO_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
TINYGO_VER := 0.26.0
TITLE = Snek Game

.PHONY: help install-tools build clean
.DEFAULT_GOAL = build

# Compilation flags
GOFLAGS = -target ./target.json -panic trap
ifeq ($(DEBUG), 1)
	GOFLAGS += -opt 1
else
	GOFLAGS += -opt z -no-debug
endif

# wasm-opt flags
WASM_OPT_FLAGS = -Oz --zero-filled-memory --strip-producers --enable-bulk-memory

help: ## 💬 This help message :)
	@figlet $@ || true
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

install-tools: ## 🔮 Install dev tools and pre-reqs
	@figlet $@ || true
	@mkdir -p bin
	@wget -q https://github.com/tinygo-org/tinygo/releases/download/v$(TINYGO_VER)/tinygo_$(TINYGO_VER)_amd64.deb -O /tmp/tinygo.deb
	@sudo dpkg -i /tmp/tinygo.deb
	@rm /tmp/tinygo.deb
	@wget -q https://github.com/aduros/wasm4/releases/latest/download/w4-linux.zip
	@unzip ./w4-linux.zip -d ./bin/
	@rm ./w4-linux.zip

build: ## 🔨 Build the game cart WASM
	@mkdir -p build
	@$(GO) build $(GOFLAGS) -o build/cart.wasm .
ifneq ($(DEBUG), 1)
ifeq (, $(shell command -v $(WASM_OPT)))
	@echo Tip: $(WASM_OPT) was not found. Install it from binaryen for smaller builds!
else
	$(WASM_OPT) $(WASM_OPT_FLAGS) build/cart.wasm -o build/cart.wasm
endif
endif

clean: ## 🧹 Clean up build artifacts
	@rm -rf $(BUILD)
	@rm -rf $(OUT)

run: ## 🚀 Run the game and start the web server
	@$(BIN)/w4 run $(BUILD)/cart.wasm --no-qr

watch: ## 👀 Run the game with reload on file change
	@$(BIN)/w4 watch --no-qr

publish: build ## 🎁 Bundle distribution files
	@$(BIN)/w4 bundle $(BUILD)/cart.wasm --html $(OUT)/index.html --title "$(TITLE)" --icon-file assets/icon.png
	@$(BIN)/w4 bundle $(BUILD)/cart.wasm --linux $(OUT)/game --title "$(TITLE)"
	@$(BIN)/w4 bundle $(BUILD)/cart.wasm --windows $(OUT)/game.exe --title "$(TITLE)"