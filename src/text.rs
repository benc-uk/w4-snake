use crate::palette;
use crate::wasm4;

pub fn shadow_text(text_v: &str, x: i32, y: i32, sc: u16, dc: u16) {
    palette::set_draw_color(dc);
    wasm4::text(text_v, x - 1, y + 1);
    palette::set_draw_color(sc);
    wasm4::text(text_v, x, y);
}
