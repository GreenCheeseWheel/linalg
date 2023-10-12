use std::time::SystemTime;
use std::env;
use linalg::Matrix;

fn main() {
    let file_path:Vec<String> = env::args().collect();
    
    let file_path = file_path.get(1).expect("You must provide a file path!");

    let mat = Matrix::from(file_path);


    if let Ok(mut nice) = mat {
        nice.swap_rows(3, 6);
        println!("NICE SWAPPED: {}", nice);
    }

    /* 
        TODO:
        IMPLEMENT BASIC MATRIX OPERATIONS (SCALAR MULTIPLICATION TO BE IMPLEMENTED, THEN ROW ADDITION)

        MAKE getEchelon FUNCTION USE OPTIMIZED PIVOTS

        CHANGE MATRIX TRANSPOSITION TO BE IN-PLACE

        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
    

}  
