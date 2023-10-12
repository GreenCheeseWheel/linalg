use std::process;
use linalg::Matrix;

fn main() {
    let mut mat = Matrix::new(2, 7);
    
    if let Err(msg) = mat.read_csv("./matrix.csv")
    {
        println!("{}", msg);
        process::exit(-1);
    }

  

    println!("{}", mat);
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
