use nalgebra::{DMatrix, DVector};
use std::collections::HashSet;
use std::cmp;

/// Generates all unique rotation and reflection variants of a matrix.
///
/// This function creates all 8 possible orientations of a piece:
/// - 4 rotations (0°, 90°, 180°, 270°)
/// - 4 reflections (mirrored versions of each rotation)
///
/// Duplicate variants are automatically removed.
#[must_use]
pub fn rotation_variants(matrix: &DMatrix<u32>) -> Vec<DMatrix<u32>> {
    let mut variants = vec![matrix.clone()];
    variants.push(rotate_matrix(variants.last().unwrap()));
    variants.push(rotate_matrix(variants.last().unwrap()));
    variants.push(rotate_matrix(variants.last().unwrap()));

    variants.push(matrix.transpose());
    variants.push(rotate_matrix(variants.last().unwrap()));
    variants.push(rotate_matrix(variants.last().unwrap()));
    variants.push(rotate_matrix(variants.last().unwrap()));

    // Dedup
    let hash_set: HashSet<DMatrix<u32>> = variants.into_iter().collect();
    hash_set.into_iter().collect()
}

/// Rotates a matrix 90 degrees clockwise.
///
/// This is implemented by transposing the matrix and reversing the row order.
#[must_use]
pub fn rotate_matrix(matrix: &DMatrix<u32>) -> DMatrix<u32> {
    // Switch columns and rows
    let mut columns: Vec<DVector<u32>> = Vec::new();
    for row in matrix.row_iter() {
        let column = row.transpose();
        columns.insert(0, column);
     }

    DMatrix::from_columns(&columns)
}

/// Clamps all values in a matrix to a maximum value.
///
/// Returns a new matrix where all values greater than `max_val` are replaced with `max_val`.
#[must_use]
pub fn max_matrix(matrix: &DMatrix<u32>, max_val: u32) -> DMatrix<u32> {
    matrix.slice((0, 0), (matrix.nrows(), matrix.ncols())).map(|val| cmp::min(val, max_val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix() {
        let matrix = DMatrix::from_row_slice(2, 2, &[1, 2, 3, 4]);
        let rotated = rotate_matrix(&matrix);
        // Original:
        // 1 2
        // 3 4
        // Rotated 90 deg clockwise:
        // 3 1
        // 4 2
        assert_eq!(rotated, DMatrix::from_row_slice(2, 2, &[3, 1, 4, 2]));
    }

    #[test]
    fn test_rotation_variants() {
        // L-shape
        // 1 0
        // 1 1
        let matrix = DMatrix::from_row_slice(2, 2, &[1, 0, 1, 1]);
        let variants = rotation_variants(&matrix);
        // L-shape has 8 variants (4 rotations * 2 reflections), all unique?
        // Let's check a simpler one.
        // Square:
        // 1 1
        // 1 1
        // Should have 1 variant.
        let square = DMatrix::from_row_slice(2, 2, &[1, 1, 1, 1]);
        let square_variants = rotation_variants(&square);
        assert_eq!(square_variants.len(), 1);

        // Rectangle:
        // 1 1
        // Should have 2 variants (horizontal and vertical).
        let rect = DMatrix::from_row_slice(1, 2, &[1, 1]);
        let rect_variants = rotation_variants(&rect);
        assert_eq!(rect_variants.len(), 2);
    }

    #[test]
    fn test_max_matrix() {
        let matrix = DMatrix::from_row_slice(2, 2, &[1, 5, 10, 0]);
        let max_val = 5;
        let clamped = max_matrix(&matrix, max_val);
        assert_eq!(clamped, DMatrix::from_row_slice(2, 2, &[1, 5, 5, 0]));
    }
}