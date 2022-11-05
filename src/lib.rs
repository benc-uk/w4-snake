#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod palette;
mod snake;
mod wasm4;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SNAKE_GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0x051f39, 0x4a2480, 0xc53a9d, 0xff8e80]);
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("whatever").update();
}
