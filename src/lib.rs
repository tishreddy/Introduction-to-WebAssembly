use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

// Import the `console.log` function from the Web APIs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct PixelEditor {
    width: usize,
    height: usize,
    pixels: Vec<u8>, // RGBA values
    current_color: [u8; 4],
}

#[wasm_bindgen]
impl PixelEditor {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> PixelEditor {
        console_log!("Creating new PixelEditor: {}x{}", width, height);
        
        let mut pixels = vec![255u8; width * height * 4]; // Initialize to white
        
        // Set alpha channel to fully opaque
        for i in (3..pixels.len()).step_by(4) {
            pixels[i] = 255;
        }
        
        PixelEditor {
            width,
            height,
            pixels,
            current_color: [0, 0, 0, 255], // Black by default
        }
    }
    
    #[wasm_bindgen]
    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.current_color = [r, g, b, a];
        console_log!("Color set to: ({}, {}, {}, {})", r, g, b, a);
    }
    
    #[wasm_bindgen]
    pub fn paint_pixel(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * 4;
            self.pixels[index] = self.current_color[0];     // Red
            self.pixels[index + 1] = self.current_color[1]; // Green
            self.pixels[index + 2] = self.current_color[2]; // Blue
            self.pixels[index + 3] = self.current_color[3]; // Alpha
        }
    }
    
    #[wasm_bindgen]
    pub fn get_pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
    
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        for i in (0..self.pixels.len()).step_by(4) {
            self.pixels[i] = 255;     // Red
            self.pixels[i + 1] = 255; // Green  
            self.pixels[i + 2] = 255; // Blue
            self.pixels[i + 3] = 255; // Alpha
        }
        console_log!("Canvas cleared");
    }
    
    #[wasm_bindgen]
    pub fn fill_gradient(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) * 4;
                // Create a gradient from top-left (dark) to bottom-right (bright)
                let intensity = ((x + y) as f32 / (self.width + self.height) as f32 * 255.0) as u8;
                self.pixels[index] = intensity;     // Red
                self.pixels[index + 1] = intensity; // Green
                self.pixels[index + 2] = intensity; // Blue
                self.pixels[index + 3] = 255;       // Alpha (fully opaque)
            }
        }
        console_log!("Gradient applied");
    }
}