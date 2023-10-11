pub mod file_reader;
pub mod matrix_product;
use std::ops;

use matrix_product::matrix_product;


pub struct Matrix
{
    pub rows:usize,
    pub cols:usize,
    pub data:Vec<f64>
}

impl Matrix {
    
    // INITIALIZES A NULL MATRIX
    pub fn new(rows:usize, cols:usize) -> Matrix {
        let data:Vec<f64> = vec![0.0; rows * cols];
        
        Matrix { 
            rows, 
            cols, 
            data 
        }
    }

    // RETURNS FALSE IF OPERATION CANNOT BE COMPLETED
    pub fn set_data(&mut self, data:Vec<f64>) -> bool
    {
        if data.len() != self.rows * self.cols
        {
            return false;
        }

        self.data = data;

        true
    }


    /*
    NOT IMPLEMENTED FOR THE MOMENT
    -
    --
    ---
    ----
    ---
    --
    -
    pub fn get_echelon(&self) -> Result<Matrix, &str>
    {
       
        let mut mat = self.data.clone();

        // GAUSS-JORDAN
        for k in 0..self.cols {

            // PIVOT ELEMENT
            let pivot = mat[k*self.cols + k];

            if pivot == 0.0
            {
                return Err("Matrix has a zero diagonal element rows");
            }

            // HERE WE GO THROUGH THE ELEMENTS BELOW THE PIVOT
            for i in k+1..self.rows
            {
                let factor = mat[i*self.cols + k] / pivot;

                // WE SET THE MULTIPLICATIVE FACTOR NEEDED TO MAKE THIS ELEMENT 0
                // AND UPDATE THE ROW AS IF WE HAD DONE AN OPERATION OF TYPE
                // ROW = ROW + x * OTHER_ROW
                for j in k..self.cols
                {
                    mat[i*self.cols + j] -= factor*mat[k*self.cols + j];
                }
            }
        
        }

        Ok(
            Matrix 
            {
                rows: self.rows,
                cols: self.cols,
                data: mat
            }
        )

    }
    */

    // SHOULD RETURN A MATRIX
    pub fn pow(&self, n:u32) -> Result<Matrix, &str>
    {
        let mut res = Matrix::new(self.rows, self.cols);
        res.set_data(self.data.clone());

        for _ in 1..n
        {
            match matrix_product::matrix_product(self, &res) {
                Ok(mat) => res = mat,
                Err(msg) => return Err(msg)
            }     
        }
       
        Ok(res)
    }

    pub fn det(&self) -> Result<f64, &str>
    {
        if self.rows != self.cols
        {
            return Err("Non-square matrix");
        }

        let n = self.rows;
        let mut result = 1.0;
        let mut mat = self.data.clone();

        // GAUSS-JORDAN
        for k in 0..n {

            // PIVOT ELEMENT
            let pivot = mat[k*n + k];

            if pivot == 0.0
            {
                return Err("Matrix has lin. dependent rows");
            }

            // HERE WE GO THROUGH THE ELEMENTS BELOW THE PIVOT
            for i in k+1..n
            {
                let factor = mat[i*n + k] / pivot;

                // WE SET THE MULTIPLICATIVE FACTOR NEEDED TO MAKE THIS ELEMENT 0
                // AND UPDATE THE ROW AS IF WE HAD DONE AN OPERATION OF TYPE
                // ROW = ROW + x * OTHER_ROW
                for j in k..n
                {
                    mat[i*n + j] = mat[i*n + j] -factor*mat[k*n + j];
                }
            }
        
        }

        // WE CALCULATE THE DETERMINANT USING THE 
        // DIAGONAL OF THIS UPPER-TRIANGULAR EQUIVALENT MATRIX
        for k in 0..n
        {
            result *= mat[k*n + k];
        }

        Ok(result)
    }

    pub fn read_csv(&mut self, file_path:&str)
    {
        if let Some(vec) = file_reader::read_csv(file_path)
        {
            self.data = vec;
        }
    }

}

impl ops::Mul<Matrix> for Matrix
{
    type Output = Result<Matrix, &'static str>;

    fn mul(self, rhs: Matrix) -> Result<Matrix, &'static str> {
        matrix_product(&self, &rhs)
    }
}