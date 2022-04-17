use nalgebra::{DMatrix, DVector};
use std::collections::HashSet;
use std::cmp;


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
    return hash_set.into_iter().collect();
}

pub fn rotate_matrix(matrix: &DMatrix<u32>) -> DMatrix<u32> {
    // Well, switch columns and rows
    let mut columns: Vec<DVector<u32>> = Vec::new();
    for row in matrix.row_iter() {
        let column = row.transpose();
        columns.insert(0, column);
     }

    DMatrix::from_columns(&columns)
}

pub fn max_matrix(matrix: &DMatrix<u32>, max_val: u32) -> DMatrix<u32> {
    matrix.slice((0, 0), (matrix.nrows(), matrix.ncols())).map(|val| cmp::min(val, max_val))
}