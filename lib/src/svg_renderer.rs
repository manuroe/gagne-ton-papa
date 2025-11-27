use nalgebra::DMatrix;
use svg::Document;
use svg::node::element::Rectangle;

/// Converts a matrix representation of a game board to an SVG string.
///
/// Each cell in the matrix is rendered as a colored rectangle in the SVG.
/// Non-zero values are rendered with their RGB color, zero values are transparent.
///
/// # Arguments
///
/// * `matrix` - A matrix where each value represents a color (24-bit RGB hex)
///
/// # Returns
///
/// An SVG document as a string
#[must_use]
pub fn svg_from_matrix(matrix: &DMatrix<u32>) -> String {
    let width = matrix.ncols();
    let height = matrix.nrows();

    let cell_size = 10;

    let mut document = Document::new()
        .set("viewBox", (0, 0, width * cell_size, height * cell_size));

    for y in 0..height {
        for x in 0..width {
            let color = matrix[(y, x)];
            let (r, g, b) = from_rgb_u32(color);
            let svg_color = if color > 0 {
                format!("rgb({r}, {g}, {b})")
            } else {
                "none".to_string()
            };
            let rect = Rectangle::new()
                .set("x", x * cell_size)
                .set("y", y * cell_size)
                .set("width", cell_size)
                .set("height", cell_size)
                .set("stroke", svg_color.clone())
                .set("fill", svg_color);

            document = document.add(rect);
        }
    }

    document.to_string()
}

/// Converts a 24-bit RGB hex color to separate R, G, B components.
///
/// # Arguments
///
/// * `c` - A 24-bit color value in format 0xRRGGBB
///
/// # Returns
///
/// A tuple of (red, green, blue) values, each in range 0-255
#[must_use]
pub const fn from_rgb_u32(c: u32) -> (u8, u8, u8) {
    let r = ((c & 0x00FF_0000u32) >> 16) as u8;
    let g = ((c & 0x0000_FF00u32) >> 8) as u8;
    let b = (c & 0x0000_00FFu32) as u8;
    (r, g, b)
}