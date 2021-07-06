use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::iter;
use std::path::Path;
// Delta force inner image format

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn as_u32(&self) -> u32 {
        return ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32);
    }
}

pub struct DFImage {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u32>,
}

impl DFImage {
    pub fn get_rgb(&self, x: u16, y: u16) -> RGBColor {
        let adress: u32 = x as u32 * y as u32 + x as u32;
        let rgb_32 = self.data[adress as usize];
        RGBColor {
            g: (rgb_32 >> 16) as u8,
            r: (rgb_32 >> 8) as u8,
            b: rgb_32 as u8,
        }
    }

    // Can load PCX, TGA, JPG
    pub fn load(path: &Path) -> Result<Self, &'static str> {
        let extension = path.extension();
        match extension {
            Some(ext) => match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                "pcx" => {
                    return DFImage::load_pcx(path);
                }
                "jpg" => {
                    return DFImage::load_jpg(path);
                }
                "tga" => {
                    return DFImage::load_tga(path);
                }
                _ => {
                    return Err("Not supported extension");
                }
            },
            None => {
                return Err("Path haven't extension");
            }
        }
    }

    fn load_pcx(path: &Path) -> Result<Self, &'static str> {
        let mut pcx = pcx::Reader::from_file(path).unwrap();

        let width = pcx.width();
        let height = pcx.height();

        let mut image = Vec::new();

        if pcx.is_paletted() {
            for _ in 0..pcx.height() {
                let mut row: Vec<u8> = iter::repeat(0).take(pcx.width() as usize).collect();
                pcx.next_row_paletted(&mut row).unwrap();
                image.push(row);
            }
        } else {
            unimplemented!("Not paletted pcx not yet supported.")
        }

        let mut palette = [0; 256 * 3];
        pcx.read_palette(&mut palette).unwrap();

        let mut u32_image: Vec<u32> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let i = image[y as usize][x as usize] as usize;
                let pcx_r = palette[i * 3 + 0];
                let pcx_g = palette[i * 3 + 1];
                let pcx_b = palette[i * 3 + 2];

                // TODO(optim): Not very effective
                u32_image.push(((pcx_r as u32) << 16) | ((pcx_g as u32) << 8) | (pcx_b as u32));
            }
        }

        return Ok(DFImage {
            width,
            height,
            data: u32_image,
        });
    }

    fn load_jpg(path: &Path) -> Result<Self, &'static str> {
        let img = ImageReader::open(path).unwrap().decode().unwrap();

        let mut u32_image: Vec<u32> = Vec::new();
        for pixel in img.as_rgb8().unwrap().pixels() {
            let jpg_r = pixel.0[0];
            let jpg_g = pixel.0[1];
            let jpg_b = pixel.0[2];

            // TODO(optim): Not very effective
            u32_image.push(((jpg_r as u32) << 16) | ((jpg_g as u32) << 8) | (jpg_b as u32));
        }
        return Ok(DFImage {
            width: img.width() as u16,
            height: img.height() as u16,
            data: u32_image,
        });
    }

    fn load_tga(_path: &Path) -> Result<Self, &'static str> {
        return Err("Can't load image");
    }
}
