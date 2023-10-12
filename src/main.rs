use std::time::SystemTime;
use std::env;
use linalg::Matrix;

fn main() {
    let file_path:Vec<String> = env::args().collect();
    
    let file_path = file_path.get(1).expect("You must provide a file path!");

    let mat = Matrix::from(file_path);


    if let Ok(mut nice) = mat {
        println!("{}", nice);
        nice.add_row(2, 1, 1.0);
        println!("{}", nice);
        
    }

    /* 
        TODO:
       
        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
    

}  
