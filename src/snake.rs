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
                Point { x: 2, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 0 },
            ],
            direction: Point { x: 1, y: 0 },
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x43);
        for &Point { x, y } in self.body.iter() {
            wasm4::rect(x * 8, y * 8, 8, 8);
        }

        set_draw_color(0x4);
        wasm4::rect(self.body[0].x * 8, self.body[0].y * 8, 8, 8);
    }

    pub fn update(&mut self) -> Option<Point> {
        if self.body.len() > 1 {
            self.body.pop()
        } else {
            None
        }
    }
}
