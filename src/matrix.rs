// Native library
use std::{ops, fmt::Display, fmt::Formatter};

// My crates
use crate::file_reader;
use crate::matrix_product::{self, matrix_product, gram_schmidt, dot_product};

/* 
    THIS MATRIX STRUCT WILL REPRESENT A DYNAMIC MATRIX
    STATIC MATRICES WILL BE ADDED LATER TO BE ABLE TO STORE IN THE STACK
*/
pub struct Matrix
{
    pub rows:usize,
    pub cols:usize,
    pub data:Vec<f64>
}


impl Matrix {
    ///////
    // ALL INSTANTIATION METHODS
    ///////
    pub fn new(rows:usize, cols:usize) -> Matrix {
        if rows <= 0 || cols <= 0
        {
            panic!("Invalid matrix instantiation. Rows and columns must be greater than zero");
        }

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

    //
    // Takes as argument any object that implements the IntoIterator trait (e.g. any vector)
    //
    pub fn from_iterator<T>(rows:usize, cols:usize, iter:T) -> Matrix 
    where T: IntoIterator<Item = f64>
    {
        let as_iter = iter.into_iter();
    
        let data:Vec<f64> = as_iter.collect();

        Matrix { rows, cols, data }
    }

    pub fn identity(order:usize) -> Matrix
    {
        let mut data:Vec<f64> = vec![0.0; order*order];

        for i in 0..order
        {
            data[i*order + i] = 1.0;
        }

        Matrix { 
            rows: order, 
            cols: order, 
            data 
        }

    }



    pub fn set_data(&mut self, data:Vec<f64>) -> bool
    {
        if data.len() != self.rows * self.cols
        {
            return false;
        }

        self.data = data;

        true
    }


    ///////
    // ALL ELEMENTARY OPERATION METHODS
    ///////

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
        if target > self.rows || row > self.rows || target == 0 || row == 0
        {
            return false;
        }

        for i in 0..self.cols
        {
            self.data[(target-1)*self.cols + i] += scalar*self.data[(row-1)*self.cols + i];
            self.data[(target-1)*self.cols + i] = self.data[(target-1)*self.cols + i];
        }

        true
    }

  
    pub fn get_row(&self, row:usize) -> Matrix
    {
        let mut row_vector:Vec<f64> = vec![]; 

        for i in 0..self.cols
        {
            row_vector.push(self.data[(row-1)*self.cols + i]);
        }


        Matrix { rows: 1, cols: self.cols, data: row_vector }
    }

    pub fn get_col(&self, col:usize) -> Matrix
    {
        let mut col_vector:Vec<f64> = vec![]; 

        for i in 0..self.rows
        {
            col_vector.push(self.data[i*self.cols + col-1]);
        }


        Matrix { rows: self.rows, cols: 1, data: col_vector }
    }


    pub fn get_echelon(&self) -> (Matrix, usize)
    {
        let mut matrix = self.clone();
        let mut row_swaps:usize = 0;
        // GAUSS-JORDAN
        for k in 0..self.rows {


            let mut pivot:f64 = 0.0;
            let mut row = k;

            for i in k..self.rows
            {
                if matrix.data[i*self.cols + k].abs() > pivot
                {
                    pivot = matrix.data[i*self.cols + k];
                    row = i;
                }
            }


            matrix.swap_rows(k+1, row+1);
            
            if k != row
            {
                row_swaps += 1;
            }

            if pivot == 0.0
            {
                continue;
            }

            // HERE WE GO THROUGH THE ELEMENTS BELOW THE PIVOT
            for i in k+1..self.rows
            {
                let factor = matrix.data[i*self.cols + k] / pivot;

                matrix.add_row(i+1, k+1, -factor);
                
            }
        
        }

        (matrix, row_swaps)

    }
    
    pub fn get_cofactor(&self, row:usize, column:usize) -> Result<f64, &str>
    {
        if row > self.rows || column > self.cols || row <= 0 || column <= 0
        {
            return Err("Invalid row or column");
        }

        if self.cols != self.rows
        {
            return Err("Tried calculating cofactor for non-square matrix");
        }

        let mut matrix = Matrix::new(self.rows-1, self.cols-1);
        let mut data:Vec<f64> = vec![];
        let mut cofactor = 0.0;

        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                if i + 1 != row && j + 1 != column
                {
                    data.push(self.data[i*self.cols + j]);
                }
            }
        }

        matrix.set_data(data);


        if let Ok(det) = matrix.det()  {
            cofactor = det;
        } 

        Ok(cofactor)
    }


    pub fn get_inverse(&self) -> Result<Matrix, &str>
    {
        if self.rows != self.cols
        {
            return  Err("Must be a square matrix");
        }

        let mut matrix = self.clone();
        let mut identity = Matrix::identity(self.rows);
        // GAUSS-JORDAN
        for k in 0..self.rows {

            let mut pivot:f64 = 0.0;
            let mut row = k;

            for i in k..self.rows
            {
                if matrix.data[i*self.cols + k].abs() > pivot
                {
                    pivot = matrix.data[i*self.cols + k];
                    row = i;
                }
            }


            
            if k != row
            {
                matrix.swap_rows(k+1, row+1);
                identity.swap_rows(k+1, row+1);
            }

            if pivot == 0.0
            {
                return Err("Tried calculating inverse for matrix with zero determinant");
            }

            // HERE WE GO THROUGH THE ELEMENTS BELOW THE PIVOT
            for i in k+1..self.rows
            {
                let factor = matrix.data[i*self.cols + k] / pivot;

                matrix.add_row(i+1, k+1, -factor);
                identity.add_row(i+1, k+1, -factor);
        
            }

            for i in 0..k
            {
                let factor = matrix.data[i*self.cols + k] / pivot;

                matrix.add_row(i+1, k+1, -factor);
                identity.add_row(i+1, k+1, -factor);
            }

        }

        for i in 0..self.cols
        {
            identity.mult_row(i+1, 1.0 / matrix.data[i*self.cols + i]);
        }

    

        Ok(identity)

    }


    //
    // EIGENVECTORS AND EIGENVALUES WILL BE CALCULATED USING 
    // GERSCHGORIN'S DISCS THEOREM
    //
    pub fn get_gerschgorin(&self) -> Result<Vec<(f64, f64)>, &str>
    {
        if self.rows != self.cols
        {
            return Err("Tried calculating eigenvalues and eigenvectors for a non-square matrix");
        }

        let mut discs:Vec<(f64, f64)> = vec![];

        for i in 0..self.rows
        {
            let disc_center = self.data[i*self.cols + i];
            let mut radius = 0.0;
            
            for j in 0..self.cols
            {
                if j != i
                {
                    radius += self.data[i*self.cols + j].abs();
                }
            }

            discs.push((disc_center, radius));
        }

        Ok(discs)
    }

    //////////////////
    //
    // IMPLEMENTATION FOR DECOMPOSITIONS BELOW
    //
    //////////////////
    
    pub fn qr_decompose(&self) -> (Matrix, Matrix)
    {
        let mut q = self.clone();

        let mut r = Matrix::new(self.rows, self.cols);
        
        // We get the columns of the matrix first
        // as a basis

        let mut basis:Vec<Matrix> = vec![];

        for i in 1..=q.cols
        {
            let row = q.get_col(i);
            basis.push(row);
        }

        let mut basis = gram_schmidt(&mut basis);
        

        (q, r)
    }


    pub fn pow(&self, n:u32) -> Result<Matrix, &str>
    {
        let mut res = self.clone();
        

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
        

        // WE CALCULATE THE DETERMINANT USING THE 
        // DIAGONAL OF THIS UPPER-TRIANGULAR EQUIVALENT MATRIX
        let matrix = self.get_echelon();
        
        for k in 0..n
        {
            result *= matrix.0.data[k*n + k];
        }

        let num_swaps = matrix.1 as u32;

        result *= (-1 as i32).pow(num_swaps) as f64;

        Ok(result)
                
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

    pub fn round(&mut self, decimal_places:i32)
    {
        let factor = 10.0_f64.powi(decimal_places);

        for i in 0..self.data.len()
        {
            self.data[i] = (self.data[i] * factor).round() / factor;
        }
    }

    //////
    //
    // VECTOR IMPLEMENTATIONS HERE
    //
    //////
    
    pub fn modulus(&self) -> f64
    {
        if self.rows != 1 && self.cols != 1
        {
            return -1.0;
        }

        let mut modulus = 0.0;

        for i in 0..self.data.len()
        {
            modulus += self.data[i] * self.data[i];
        }

        modulus.sqrt()
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
    type Output =   Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {

        match matrix_product(self, rhs) {
            Ok(matrix) => return matrix,
            Err(msg) => panic!("ERROR MULTIPLYING MATRICES: {}", msg)
        }
       
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

impl ops::Sub<&Matrix> for &Matrix {
    
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        
        if self.rows != rhs.rows || self.cols != rhs.cols
        {
            panic!("Matrices must have the same dimensions!");
        }

        let mut matrix = self.clone();

        for i in 0..self.data.len()
        {
            matrix.data[i] -= rhs.data[i];
        }

        matrix
    }

}

//////
//
// QOL IMPLEMENTATIONS HERE
//
//////

impl ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}