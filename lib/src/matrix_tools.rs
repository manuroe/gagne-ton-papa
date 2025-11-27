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