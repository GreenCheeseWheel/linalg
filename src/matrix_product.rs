use crate::matrix::Matrix;

pub fn dot_product(vec1:&Vec<f64>, vec2:&Vec<f64>) -> Result<f64, &'static str>
{
    if vec1.len() != vec2.len()
    {
        return Err("Vectors must have the same dimension");
    }

    

    let mut res = 0.0;

    for i in 0..vec1.len()
    {
        res += vec1[i] * vec2[i];
    }

    Ok(res)
}

pub fn matrix_product(mat1:&Matrix, mat2:&Matrix) -> Result<Matrix, &'static str>
{
    if mat1.cols != mat2.rows
    {
        return Err("Matrix dimensions don't match");
    }

    let mut mat:Vec<f64> = vec![0.0; mat1.rows * mat2.cols];

    for i in 0..mat1.rows
    {
        for j in 0..mat2.cols
        {
            let mut row:Vec<f64> = vec![];

            let mut col:Vec<f64> = vec![];

            for k in 0..mat1.cols
            {
                row.push(mat1.data[i*mat1.cols + k]);
            }

            for k in 0..mat2.rows
            {
                col.push(mat2.data[j + k*mat2.cols]);
            }

            match dot_product(&row, &col) {
                Ok(res) => mat[i*mat2.cols + j] = res,
                Err(msg) => return Err(msg)
            }

        }
    }

    Ok(
        Matrix
        {
            rows: mat1.rows,
            cols: mat2.cols,
            data: mat
        }
    )

}