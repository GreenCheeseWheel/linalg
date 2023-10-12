use std::process;

use linalg::Matrix;

fn main() {
    let mut mat = Matrix::new(3, 4);
    let mut mat2 = Matrix::new(4, 3);

    println!("PRIMER ARCHIVO");
    if let Err(msg) = mat.read_csv("./matrix.csv")
    {
        println!("{}", msg);
        process::exit(-1);
    }


    println!("PRIMER ARCHIVO");
    if let Err(msg) = mat2.read_csv("./matrix2.csv")
    {
        println!("{}", msg);
        process::exit(-1);
    }



    println!("MATRIZ UNO: \n {}", mat);
    println!("MATRIZ DOS: \n {}", mat2);

    println!("{}", (&mat2 * &mat).unwrap() );

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
