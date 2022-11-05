use crate::palette::{self, set_draw_color};
use crate::snake::{Point, Snake};
use crate::wasm4::{self, text};
use fastrand::Rng;

pub struct Game {
    rng: Rng,
    snake: Snake,
    frame_count: u32,
    prev_gamepad: u8,
    fruit: Point,
    is_game_over: bool,
    score: u32,
    speed: u32,
}

const FRUIT_SPRITE: [u8; 16] = [
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            frame_count: 0,
            prev_gamepad: 0,
            fruit: Point { x: 10, y: 10 },
            rng: Rng::with_seed(235),
            is_game_over: false,
            score: 0,
            speed: 18,
        }
    }

    pub fn update(&mut self) {
        if self.is_game_over {
            palette::set_draw_color(0x4);
            text("   SNEK IS DEAD\nPRESS R TO RESTART", 10, 70);
            return;
        }

        self.frame_count += 1;

        self.input();

        if self.snake.is_dead() {
            self.is_game_over = true;
            wasm4::tone(262 | (23 << 16), 27 | (10 << 8), 60, wasm4::TONE_PULSE1);
        }

        if self.frame_count % self.speed == 0 {
            let dropped_pos = self.snake.update();
            if self.snake.body[0] == self.fruit {
                self.snake.body.push(dropped_pos);
                self.fruit.x = self.rng.i32(1..19);
                self.fruit.y = self.rng.i32(1..19);
                self.score += 10;
                wasm4::tone(262 | (723 << 16), 2 | (10 << 8), 60, wasm4::TONE_PULSE2);
            }
        }

        self.snake.draw();

        set_draw_color(0x4320);
        wasm4::blit(
            &FRUIT_SPRITE,
            self.fruit.x * 8,
            self.fruit.y * 8,
            8,
            8,
            wasm4::BLIT_2BPP,
        );

        palette::set_draw_color(0x2);
        wasm4::rect(0, 0, 160, 8);

        palette::set_draw_color(0x1);
        text(&format!("SCORE: {}", self.score), 2, 0);
        palette::set_draw_color(0x3);
        text(&format!("SCORE: {}", self.score), 3, 1);

        if self.frame_count % 400 == 0 {
            self.speed -= 1;
            if self.speed < 2 {
                self.speed = 2;
            }
        }
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.snake.left();
        }

        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.snake.right();
        }

        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.snake.up();
        }

        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.snake.down();
        }

        self.prev_gamepad = gamepad;
    }
}
