mod image;

use std::fs::File;
use std::mem;
use std::path::Path;

use minifb::{Key, ScaleMode, Window, WindowOptions};

// PCX renderer
// http://bespin.org/~qz/pc-gpe/pcx.txt

/*

Example for writing PCX image:

// Create 5x5 RGB file.
let mut writer = pcx::WriterRgb::create_file("test.pcx", (5, 5), (300, 300)).unwrap();
for y in 0..5 {
    // Write 5 green pixels.
    writer.write_row(&[0, 255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0]);
}
writer.finish().unwrap();

 */

fn main() {
    let mut reader = pcx::Reader::from_file("data/dfd1/dfd1_d.pcx").unwrap();

    println!(
        "width = {}, height = {}, paletted = {}",
        reader.width(),
        reader.height(),
        reader.is_paletted()
    );

    let mut pcx_buf: Vec<u8> = Vec::new();

    for _ in 0..reader.height() {
        if reader.is_paletted() {
            let mut row_buf = vec![0; reader.width() as usize];
            reader.next_row_paletted(&mut row_buf).unwrap();
            pcx_buf.append(&mut row_buf.clone());
        } else {
            // call reader.next_row_rgb(...) or reader.next_row_rgb_separate(...) to read next row
        }
    }

    let u32_buffer: Vec<u32> = pcx_buf
        .chunks_exact(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    let width: usize = (reader.width()) as usize;
    let height: usize = (u32_buffer.len() as f32 / width as f32) as usize;
    let _buf_size = width * height;

    let mut window = Window::new(
        "Open Delta",
        (width as f32 / 3 as f32) as usize,
        height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&u32_buffer, width, height)
            .unwrap();
    }
}
