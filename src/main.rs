use linalg::{Matrix, matrix_product};

fn main() {
    let mut mat = Matrix::new(3, 3);
    let mut mat2 = Matrix::new(3, 3);
    
    mat.read_csv("./matrix.csv");
    mat2.read_csv("./matrix.csv");

    if let Ok(mat) = &mat * &mat2
    {
        println!("MATRIZ ES: {}", mat);
    }

    println!("PRIMERA MATRIZ \n {}", mat);
    /*
        TODO:

        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
    

}  
