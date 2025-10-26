use image::{ImageBuffer, RgbImage};
use crate::math::vector3::Vector3;

pub type Color = Vector3;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }
    
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index]
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn save_to_ppm(&self, filename: &str) {
        let mut img: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = self.get_pixel(x as usize, y as usize);
            let r = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        
        img.save(filename).expect("Error al guardar imagen PPM");
    }
    
    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
}