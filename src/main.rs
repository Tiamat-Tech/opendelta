mod dfimage;

use std::fs::File;
use std::mem;
use std::path::Path;

use dfimage::{DFImage, RGBColor};
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
    height_image: dfimage::DFImage,
}

pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

// Voxel
pub fn render_scene(
    heightmap: dfimage::DFImage,
    point: Point2D,
    height: f32,
    horizon: f32,
    scale_height: f32,
    distance: f32,
    screen_width: u32,
    screen_height: u32,
) -> Vec<u32> {
    let mut result_buffer = vec![0; screen_width as usize * screen_height as usize];
    let mut z = distance as i32;
    while z > 1 {
        // From back to front
        let mut pleft = Point2D {
            x: -z + point.x,
            y: -z + point.y,
        };
        let pright = Point2D {
            x: z + point.x,
            y: -z + point.y,
        };

        let dx = (pright.x - pleft.x) as f32 / screen_width as f32;

        for i in 0..screen_width {
            let heightmap_clr = heightmap.get_rgb(pleft.x as u16, pleft.y as u16);
            let height_on_screen =
                (height - heightmap_clr.r as f32) / z as f32 * scale_height + horizon;

            draw_vertical_line(&mut result_buffer, i, height_on_screen, screen_height as f32, heightmap_clr);

            pleft.x += dx as i32;
        }

        z -= 1;
    }

    return result_buffer;
}

// TODO: color as rgbcolor, now its heightmap value
pub fn draw_vertical_line(buffer: &mut Vec<u32>, index: u32, mut ytop: f32, ybottom: f32, color: RGBColor) {
    if ytop >= ybottom {
        return;
    }

    if ytop < 0.0 {
        ytop = 0.0;
    }

    // if screen.width != 700:

    for j in ytop.floor() as u32 .. ybottom.floor() as u32
    {
        let x = index + 1024;
        let adress = x * j+ x;
        buffer[adress as usize] = color.as_u32();
    }

    // for j in range(math.floor(ytop), math.floor(ybottom)):
    //     screenmap[x+1024, j] = rgb
// else:
//     for j in range(math.floor(ytop), math.floor(ybottom)):
//         screenmap[x, j] = rgb
}

fn main() {
    // let clouds01 = image::DFImage::load("data/dfd1/CLOUDS01.PCX").unwrap();
    let dfd1_c = dfimage::DFImage::load(Path::new("data/dfd1/DFD1_C.JPG")).unwrap();
    // let dfd1_cm = image::DFImage::load(Path::new("data/dfd1/DFD1_CM.TGA")).unwrap();
    let dfd1_d = dfimage::DFImage::load(Path::new("data/dfd1/DFD1_D.PCX")).unwrap();
    // let dfd1_dm = image::DFImage::load("data/dfd1/DFD1_DM.PCX").unwrap();
    // let dfd1_m = image::DFImage::load("data/dfd1/DFD1_M.PCX").unwrap();
    // let ripple1 = image::DFImage::load("data/dfd1/RIPPLE1.PCX").unwrap();
    // let skygrd01 = image::DFImage::load("data/dfd1/SKYGRD01.TGA").unwrap();

    let screen_width = 800;
    let screen_height = 600;

    let mut window = Window::new(
        "Open Delta",
        screen_width,
        screen_height,
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

    let rendered_scene = render_scene(
        dfd1_d,
        Point2D { x: 0, y: 0 },
        50.0,
        120.0,
        120.0,
        300.0,
        screen_width as u32,
        screen_height as u32,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &rendered_scene,
                screen_width as usize,
                screen_height as usize,
            )
            .unwrap();
    }
}
