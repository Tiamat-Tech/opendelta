use std::fs::File;
use std::mem;
use std::path::Path;

use minifb::{Key, Window, WindowOptions};

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

    let row_buf_size: u32 = reader.width() as u32;
    let file_buf_size: u32 = row_buf_size * reader.height() as u32;

    let mut row_buf = vec![0; row_buf_size as usize];
    let mut pcx_buf: Vec<u8> = vec![0; file_buf_size as usize];

    for y in 0..reader.height() {
        if reader.is_paletted() {
            reader.next_row_paletted(&mut row_buf);
            pcx_buf.append(&mut row_buf.clone());
        } else {
            // call reader.next_row_rgb(...) or reader.next_row_rgb_separate(...) to read next row
        }
    }

    let WIDTH: usize = reader.width() as usize;
    let HEIGHT: usize = reader.height() as usize;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Open Delta", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut pcx_iter = 0;

        for i in buffer.iter_mut() {
            // dark orange
            // let r = 50;
            // let g = 168;
            // let b = 82;

            if (pcx_iter + 3 > pcx_buf.len()) {
                break;
            }

            let r: u32 = pcx_buf[pcx_iter] as u32;
            let g: u32 = pcx_buf[pcx_iter + 1] as u32;
            let b: u32 = pcx_buf[pcx_iter + 2] as u32;

            *i = r << 16 | g << 8 | b;

            pcx_iter += 2;
        }

        // window.update
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
