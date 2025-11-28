use nalgebra::DMatrix;
use svg::Document;
use svg::node::element::{Line, Rectangle};

/// Converts a matrix representation of a game board to an SVG string.
///
/// Each cell in the matrix is rendered as a colored rectangle in the SVG.
/// Non-zero values are rendered with their RGB color, zero values are transparent.
/// Borders are drawn only around the outline of each piece (connected cells of the same color).
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
            if color == 0 {
                continue;
            }

            let (r, g, b) = from_rgb_u32(color);
            let svg_color = format!("rgb({r}, {g}, {b})");

            // Draw the filled cell without stroke
            let rect = Rectangle::new()
                .set("x", x * cell_size)
                .set("y", y * cell_size)
                .set("width", cell_size)
                .set("height", cell_size)
                .set("fill", svg_color)
                .set("stroke", "none"); // No stroke on the rect itself

            document = document.add(rect);

            // Draw borders only if the neighbor is different
            let border_color = "black";
            let border_opacity = 0.3;
            let border_width = 0.5;

            // Top
            if y == 0 || matrix[(y - 1, x)] != color {
                let line = Line::new()
                    .set("x1", x * cell_size)
                    .set("y1", y * cell_size)
                    .set("x2", (x + 1) * cell_size)
                    .set("y2", y * cell_size)
                    .set("stroke", border_color)
                    .set("stroke-opacity", border_opacity)
                    .set("stroke-width", border_width);
                document = document.add(line);
            }

            // Bottom
            if y == height - 1 || matrix[(y + 1, x)] != color {
                let line = Line::new()
                    .set("x1", x * cell_size)
                    .set("y1", (y + 1) * cell_size)
                    .set("x2", (x + 1) * cell_size)
                    .set("y2", (y + 1) * cell_size)
                    .set("stroke", border_color)
                    .set("stroke-opacity", border_opacity)
                    .set("stroke-width", border_width);
                document = document.add(line);
            }

            // Left
            if x == 0 || matrix[(y, x - 1)] != color {
                let line = Line::new()
                    .set("x1", x * cell_size)
                    .set("y1", y * cell_size)
                    .set("x2", x * cell_size)
                    .set("y2", (y + 1) * cell_size)
                    .set("stroke", border_color)
                    .set("stroke-opacity", border_opacity)
                    .set("stroke-width", border_width);
                document = document.add(line);
            }

            // Right
            if x == width - 1 || matrix[(y, x + 1)] != color {
                let line = Line::new()
                    .set("x1", (x + 1) * cell_size)
                    .set("y1", y * cell_size)
                    .set("x2", (x + 1) * cell_size)
                    .set("y2", (y + 1) * cell_size)
                    .set("stroke", border_color)
                    .set("stroke-opacity", border_opacity)
                    .set("stroke-width", border_width);
                document = document.add(line);
            }
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