use crate::math::vector3::Vector3;

#[derive(Clone, Debug)]
pub struct Texture {
    width: u32,
    height: u32,
    data: Vec<Vector3>,
    tile_u: f32,
    tile_v: f32,
}

impl Texture {
    pub fn solid_color(color: Vector3) -> Self {
        Self {
            width: 1,
            height: 1,
            data: vec![color],
            tile_u: 1.0,
            tile_v: 1.0,
        }
    }

    pub fn with_tiling(mut self, u: f32, v: f32) -> Self {
        self.tile_u = u;
        self.tile_v = v;
        self
    }

    pub fn from_image(_path: &str) -> Result<Self, String> {
        use std::path::Path;
    let path = Path::new(_path);
        if !path.exists() {
            return Err(format!("Texture file not found: {}", _path));
        }

        let dynimg = match image::open(path) {
            Ok(img) => img,
            Err(_) => {
                let bytes = std::fs::read(path).map_err(|e| format!("Failed to read image bytes {}: {}", _path, e))?;
                if let Ok(img) = image::load_from_memory(&bytes) {
                    img
                } else {
                    let fmts = [image::ImageFormat::Jpeg, image::ImageFormat::Png, image::ImageFormat::WebP, image::ImageFormat::Bmp, image::ImageFormat::Tga, image::ImageFormat::Gif];
                    let mut found = None;
                    for fmt in &fmts {
                        if let Ok(img) = image::load_from_memory_with_format(&bytes, *fmt) {
                            found = Some(img);
                            break;
                        }
                    }
                    if let Some(img) = found { img } else {
                        return Err(format!("Failed to decode image {}: unknown or corrupt format", _path));
                    }
                }
            }
        };

        let img = dynimg.to_rgba8();
        let (w, h) = img.dimensions();
        let mut data = Vec::with_capacity((w * h) as usize);

        for y in 0..h {
            for x in 0..w {
                let p = img.get_pixel(x, y);
                let r = p[0] as f32 / 255.0;
                let g = p[1] as f32 / 255.0;
                let b = p[2] as f32 / 255.0;
                data.push(Vector3::new(r, g, b));
            }
        }

        Ok(Self { width: w, height: h, data, tile_u: 1.0, tile_v: 1.0 })
    }

    pub fn sample(&self, mut u: f32, mut v: f32) -> Vector3 {
        if self.width == 1 && self.height == 1 {
            return self.data[0];
        }

        u = (u * self.tile_u) % 1.0;
        v = (v * self.tile_v) % 1.0;

        if u < 0.0 { u += 1.0; }
        if v < 0.0 { v += 1.0; }

        let x = (u * (self.width - 1) as f32) as u32;
        let y = (v * (self.height - 1) as f32) as u32;
        let index = (y * self.width + x) as usize;

        self.data.get(index).copied().unwrap_or(Vector3::new(1.0, 0.0, 1.0))
    }

    pub fn checkerboard(size: u32, color1: Vector3, color2: Vector3) -> Self {
        let mut data = Vec::new();
        for y in 0..size {
            for x in 0..size {
                let color = if (x + y) % 2 == 0 { color1 } else { color2 };
                data.push(color);
            }
        }

        Self {
            width: size,
            height: size,
            data,
            tile_u: 1.0,
            tile_v: 1.0,
        }
    }
}