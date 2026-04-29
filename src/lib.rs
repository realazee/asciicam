use wasm_bindgen::prelude::*;

const CHARACTER_SET: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const ABSOLUTE_MAXIMUM_BRIGHTNESS: u8 = 255;

#[wasm_bindgen]
pub fn frame_to_ascii(
    rgba: &[u8],
    width: u32,
    height: u32,
    max_brightness: u8,
    min_brightness: u8,
    invert: bool,
) -> String {
    let chars = CHARACTER_SET.as_bytes();
    let charset_len = chars.len();
    let w = width as usize;
    let h = height as usize;
    let mut out = String::with_capacity((w + 1) * h);
    for y in 0..h {
        if y > 0 {
            out.push('\n');
        }
        for x in 0..w {
            let i = (y * w + x) * 4;
            let r = rgba[i] as u32;
            let g = rgba[i + 1] as u32;
            let b = rgba[i + 2] as u32;
            let mut brightness = ((r * 299 + g * 587 + b * 114) / 1000) as u8;
            if invert {
                brightness = ABSOLUTE_MAXIMUM_BRIGHTNESS - brightness;
            }
            if brightness > max_brightness || brightness < min_brightness || brightness == 0 {
                out.push(' ');
            } else {
                let idx = (charset_len as f32
                    * (brightness as f32 / ABSOLUTE_MAXIMUM_BRIGHTNESS as f32))
                    as usize;
                out.push(chars[idx.min(charset_len - 1)] as char);
            }
        }
    }
    out
}
