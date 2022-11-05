use crate::snake::Snake;
//use crate::wasm4;

pub struct Game {
    snake: Snake,
    frame_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count % 7 == 0 {
            self.snake.update();
        }

        self.snake.draw();
    }
}
