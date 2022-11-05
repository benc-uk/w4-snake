use crate::palette::set_draw_color;
use crate::wasm4;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Point,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Point { x: 2, y: 5 },
                Point { x: 1, y: 5 },
                Point { x: 0, y: 5 },
            ],
            direction: Point { x: 1, y: 0 },
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x32);
        for &Point { x, y } in self.body.iter() {
            wasm4::rect(x * 8, y * 8, 8, 8);
        }

        set_draw_color(0x4);
        wasm4::rect(self.body[0].x * 8, self.body[0].y * 8, 8, 8);
    }

    pub fn update(&mut self) -> Point {
        self.body.insert(
            0,
            Point {
                x: (self.body[0].x + self.direction.x),
                y: (self.body[0].y + self.direction.y),
            },
        );

        if self.body[0].x < 0 {
            self.body[0].x = 19;
        }
        if self.body[0].x > 19 {
            self.body[0].x = 0;
        }
        if self.body[0].y > 19 {
            self.body[0].y = 1;
        }

        if self.body[0].y < 1 {
            self.body[0].y = 19;
        }

        if let Some(last_pos) = self.body.pop() {
            last_pos
        } else {
            Point { x: 0, y: 0 }
        }
    }

    pub fn down(&mut self) {
        if self.direction.y != -1 {
            self.direction = Point { x: 0, y: 1 };
        }
    }

    pub fn up(&mut self) {
        if self.direction.y != 1 {
            self.direction = Point { x: 0, y: -1 };
        }
    }

    pub fn left(&mut self) {
        if self.direction.x != 1 {
            self.direction = Point { x: -1, y: 0 };
        }
    }

    pub fn right(&mut self) {
        if self.direction.x != -1 {
            self.direction = Point { x: 1, y: 0 };
        }
    }

    pub fn is_dead(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .any(|&body_section| body_section == self.body[0])
    }
}
