use std::time::SystemTime;
use linalg::Matrix;

fn main() {
    let mat = Matrix::from("./iris.csv");

    if let Ok(nice) = mat {
        println!("{}", nice.get_echelon().unwrap());
    }

    /* 
        TODO:
        CHANGE MATRIX TRANSPOSITION TO BE IN-PLACE

        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
    

}  
