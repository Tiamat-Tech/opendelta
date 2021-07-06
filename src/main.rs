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

pub struct DFTerrain {
    height_image: image::DFImage,
}

pub struct Point2D {
    x: i32,
    y: i32,
}

// Voxel
pub fn render_scene(
    point: Point2D,
    height: f32,
    scale_height: f32,
    distance: f32,
    screen_width: u32,
    screen_height: u32,
) {
    let mut z = distance;
    while z > 1.0 {
        // From back to front

        z -= 1.0;
    }
}

fn main() {
    // let clouds01 = image::DFImage::load("data/dfd1/CLOUDS01.PCX").unwrap();
    let dfd1_c = image::DFImage::load(Path::new("data/dfd1/DFD1_C.JPG")).unwrap();
    // let dfd1_cm = image::DFImage::load(Path::new("data/dfd1/DFD1_CM.TGA")).unwrap();
    let dfd1_d = image::DFImage::load(Path::new("data/dfd1/DFD1_D.PCX")).unwrap();
    // let dfd1_dm = image::DFImage::load("data/dfd1/DFD1_DM.PCX").unwrap();
    // let dfd1_m = image::DFImage::load("data/dfd1/DFD1_M.PCX").unwrap();
    // let ripple1 = image::DFImage::load("data/dfd1/RIPPLE1.PCX").unwrap();
    // let skygrd01 = image::DFImage::load("data/dfd1/SKYGRD01.TGA").unwrap();

    let mut window = Window::new(
        "Open Delta",
        dfd1_cm.width as usize,
        dfd1_cm.height as usize,
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
            .update_with_buffer(
                &dfd1_cm.data,
                dfd1_cm.width as usize,
                dfd1_cm.height as usize,
            )
            .unwrap();
    }
}
