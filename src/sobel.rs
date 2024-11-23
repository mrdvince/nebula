use crate::types::Rgba;

// Sobel operators
const SOBEL_X: [[i32; 3]; 3] = [
    [-1, 0, 1],
    [-2, 0, 2],
    [-1, 0, 1],
];

const SOBEL_Y: [[i32; 3]; 3] = [
    [-1, -2, -1],
    [0, 0, 0],
    [1, 2, 1],
];

pub fn rgba_to_gray(rgba: Rgba) -> u8 {
    let r = ((rgba >> 0) & 0xFF) as f32;   // R is in first byte
    let g = ((rgba >> 8) & 0xFF) as f32;   // G is in second byte
    let b = ((rgba >> 16) & 0xFF) as f32;  // B is in third byte
    // Standard grayscale conversion weights
    (0.299 * r + 0.587 * g + 0.114 * b) as u8
}

pub fn gray_to_rgba(gray: u8) -> Rgba {
    let gray = gray as u32;
    // Put the same gray value in R,G,B channels and set A to 255
    (gray) | (gray << 8) | (gray << 16) | (0xFF << 24)
}

pub fn apply_sobel(gray_pixels: &[u8], width: usize, height: usize, x: usize, y: usize) -> u8 {
    if x == 0 || x >= width - 1 || y == 0 || y >= height - 1 {
        return 0;
    }

    let mut gx = 0;
    let mut gy = 0;

    // Apply Sobel operators
    for i in 0..3 {
        for j in 0..3 {
            let pixel_idx = (y + i - 1) * width + (x + j - 1);
            let pixel = gray_pixels[pixel_idx] as i32;
            
            gx += pixel * SOBEL_X[i][j];
            gy += pixel * SOBEL_Y[i][j];
        }
    }

    // Compute gradient magnitude
    let mag = ((gx * gx + gy * gy) as f32).sqrt();
    // Normalize to 0-255
    (mag.min(255.0)) as u8
}