pub mod file_reader;
pub mod matrix_product;
use std::{ops, fmt::Display, fmt::Formatter};

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

    pub fn from(file_path:&str) -> Result<Matrix, &str>
    {
        if let Some(matrix_info)  =  file_reader::read_csv(file_path) {
           return Ok(
            Matrix {
                rows: matrix_info.1,
                cols: matrix_info.2,
                data: matrix_info.0
            }
            );
        } 

        Err("Could not read file")

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

    pub fn swap_rows(&mut self, r1:usize, r2:usize) -> bool
    {
        if r1 > self.rows || r2 > self.rows
        {
            return false;
        }

        for i in 0..self.cols
        {
            let stored = self.data[(r1-1)*self.cols + i];
            
            self.data[(r1-1)*self.cols + i] = self.data[(r2-1)*self.cols + i];
            
            self.data[(r2-1)*self.cols + i] = stored;   
        }

        return true;

    }

    pub fn mult_row(&mut self, row:usize, scalar:f64) -> bool
    {
        if row > self.rows || scalar == 0.0
        {
            return false;
        }

        for i in 0..self.cols
        {
            self.data[(row-1)*self.cols + i] *= scalar;
        }

        true
    }

    pub fn add_row(&mut self, target:usize, row:usize, scalar:f64) -> bool
    {
        if target > self.rows || row > self.rows
        {
            return false;
        }

        for i in 0..self.cols
        {
            self.data[(target-1)*self.cols + i] += scalar*self.data[(row-1)*self.cols + i];
        }

        true
    }

  
    pub fn get_echelon(&self) -> Result<Matrix, &str>
    {
       
        let mut mat = self.data.clone();

        // GAUSS-JORDAN
        for k in 0..self.rows {

            if k >= self.cols
            {
                break;
            }

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
                    if j <= k
                    {
                        mat[i*self.cols + j] = 0.0;
                        continue;
                    }

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
        
        if let Ok(matrix) = self.get_echelon()
        {
            // WE CALCULATE THE DETERMINANT USING THE 
            // DIAGONAL OF THIS UPPER-TRIANGULAR EQUIVALENT MATRIX
            for k in 0..n
            {
                result *= matrix.data[k*n + k];
            }

            return Ok(result);
        }
     
        Ok(0.0)
        
    }

    pub fn read_csv(&mut self, file_path:&str) -> Result<bool, String>
    {
        if let Some(vec) = file_reader::read_csv(file_path)
        {   
            if vec.0.len() != self.rows * self.cols
            {
                return Err(format!("Invalid matrix dimensions. Matrix contains {} elements while csv contains {} elements", self.rows * self.cols, vec.0.len()));
            }

            self.data = vec.0;

            return Ok(true);
        }

        Err(String::from("Could not read csv file"))
    }

    pub fn transpose(&mut self)
    {
        let mut mat = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                mat.data[j*self.rows + i] = self.data[i*self.cols + j];
            }
        }        

        let old_rows = self.rows;
        self.rows = self.cols;
        self.cols = old_rows;
        self.data = mat.data;


    }


}



//////
//
// DISPLAY IMPLEMENTATIONS HERE
//
//////

impl Display for Matrix
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        for i in 0..self.rows
        {
            if let Err(msg) = write!(f, "\n R{} <-> ", i+1)
            {
                return  Err(msg);
            }

            for j in 0..self.cols
            {
                if let Err(msg) = write!(f, " {} ", self.data[i*self.cols + j])
                {
                    return Err(msg);
                }
            }

            if let Err(msg) = write!(f, "\n")
            {
                return Err(msg);
            }
        }

        writeln!(f, "\n")
    }
}


//////
//
// CLONE AND COPY IMPLEMENTATIONS HERE
//
//////

impl Clone for Matrix
{
    fn clone(&self) -> Self {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone()
        }
    }
}


//////
//
// ARITHMETIC IMPLEMENTATIONS HERE
//
//////


impl ops::Mul<f64> for &Matrix
{
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut mat = self.clone();

        for i in 0..self.data.len()
        {
            mat.data[i] *= rhs;
        }
        
        mat
    }
}

impl ops::Mul<&Matrix> for f64
{
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut mat = rhs.clone();

        for i in 0..rhs.data.len()
        {
            mat.data[i] *= self;
        }
        
        mat
    }
}




impl ops::Mul<&Matrix> for &Matrix
{
    type Output = Result<Matrix, &'static str>;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        matrix_product(self, rhs)
    }
}

impl ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output{
        
        if self.rows != rhs.rows || self.cols != rhs.cols
        {
            panic!("Matrices must have matching dimensions");
        }

        let mut mat = Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.clone(),
        };

        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                mat.data[i*self.cols + j] = self.data[i*self.cols + j] + rhs.data[i*self.cols + j];
            }
        }

        
        mat
        
    }

}