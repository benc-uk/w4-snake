use crate::snake::Snake;
//use crate::wasm4;

pub struct Game {
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
        }
    }

    pub fn update(&mut self) {
        self.snake.update();
        self.snake.draw();
    }
}
