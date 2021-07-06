use std::path::Path;
use std::{io, iter};

// Delta force inner image format
pub struct DFImage {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u32>,
}

impl DFImage {
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
                _ => {
                    return Err("Not supported extension");
                }
            },
            None => {
                return Err("Path haven't extension");
            }
        }
        return Err("Can't load image");
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
            data: u32_image
        });
    }

    fn load_jpg(path: &Path) -> Result<Self, &'static str> {
        return Err("Can't load image");
    }
}
