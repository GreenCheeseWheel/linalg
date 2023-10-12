use linalg::Matrix;

fn main() {
    let mut mat = Matrix::new(2, 7);
    
    mat.read_csv("./matrix.csv");
  

    println!("{}", mat);
    /* 
        TODO:

        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
    

}  
