use crate::matrix::Matrix;

pub fn solve_system(matrix: &Matrix, vector: &Matrix) -> Option<Matrix> {
    if (vector.rows != 1 && vector.cols != 1) || matrix.cols > matrix.rows {
        return None;
    }

    if let Ok(inverse) = matrix.get_inverse() {
        return Some(&inverse * vector);
    }

    None
}
