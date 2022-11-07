use crate::palette::{self, set_draw_color};
use crate::snake::{Point, Snake};
use crate::text;
use crate::wasm4::{self, TONE_TRIANGLE};
use fastrand::Rng;

pub struct Game {
    rng: Rng,
    snake: Snake,
    frame_count: u32,
    prev_gamepad: u8,
    fruit: Point,
    is_game_over: bool,
    is_title_screen: bool,
    score: u32,
    high_score: u32,
    speed: u32,
}

const SPEED_TIME: u32 = 500;
const FRUIT_SPRITE: [u8; 16] = [
    0xff, 0xff, 0xff, 0xff, 0xf5, 0x5f, 0xe5, 0x07, 0xe5, 0x47, 0xe9, 0x57, 0xfa, 0xaf, 0xff, 0xff,
];

impl Game {
    pub fn new() -> Self {
        let high_score_val = unsafe {
            let mut buffer = [0u8; core::mem::size_of::<u32>()];
            wasm4::diskr(buffer.as_mut_ptr(), buffer.len() as u32);
            u32::from_le_bytes(buffer)
        };

        Self {
            snake: Snake::new(),
            frame_count: 0,
            prev_gamepad: 0,
            fruit: Point { x: 10, y: 10 },
            rng: Rng::with_seed(235),
            is_game_over: false,
            score: 0,
            speed: 18,
            is_title_screen: true,
            high_score: high_score_val,
        }
    }

    pub fn new_game(&mut self) {
        self.is_game_over = false;
        self.score = 0;
        self.speed = 18;
        self.snake = Snake::new();
        self.fruit = Point { x: 10, y: 10 };
        self.frame_count = 0;
        self.prev_gamepad = 0;
    }

    pub fn update(&mut self) {
        let pressed = self.input();

        if self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                self.new_game();
                self.is_title_screen = false;
                wasm4::tone(462 | (623 << 16), 10 | (30 << 8), 100, TONE_TRIANGLE);
            }

            text::shadow_text("Snek Gaem", 45, 30, 0x4, 0x2);
            palette::set_draw_color(0x3);
            wasm4::text(format!("High Score: {}", self.high_score), 25, 100);
            text::shadow_text(
                "INSTRACTION:\n- eat foods\n- get lang\n- win score",
                4,
                120,
                0x2,
                0x1,
            );
            text::shadow_text("PRESS x TO START", 18, 80, 0x3, 0x2);
            return;
        }

        if self.is_game_over && !self.is_title_screen {
            if pressed & wasm4::BUTTON_1 != 0 {
                if self.score > self.high_score {
                    unsafe {
                        let game_data_bytes = self.score.to_le_bytes();
                        wasm4::diskw(game_data_bytes.as_ptr(), core::mem::size_of::<u32>() as u32);
                    }
                    self.high_score = self.score;
                }
                self.is_title_screen = true;
            }
            text::shadow_text("SNEK IS DEAD\n   PRESS x", 30, 70, 0x3, 0x2);
            return;
        }

        self.frame_count += 1;

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

        set_draw_color(0x0234);
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

        text::shadow_text(&format!("SCORE: {}", self.score), 2, 0, 0x3, 0x1);

        if self.frame_count % SPEED_TIME == 0 {
            self.speed -= 1;
            if self.speed < 2 {
                self.speed = 2;
            }
        }
    }

    pub fn input(&mut self) -> u8 {
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
        return just_pressed;
    }
}
