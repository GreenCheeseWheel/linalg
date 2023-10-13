use std::time::SystemTime;
use std::env;
use linalg::Matrix;

/*
    REGLAS PARA EL DESARROLLO DE LA LIBRERIA

    1) LAS OPERACIONES MATRICIALES CON SOBRECARGA DE OPERADORES DEVOLVERAN
    EL RESULTADO ESPERADO O ENTRARÁN EN PANICO. JAMÁS DEVOLVERAN UN ENUM DE TIPO RESULT

    2) CUALQUIER FUNCIÓN QUE REALICE UNA OPERACIÓN ARITMÉTICA SOBRE UNA O MÁS MATRICES
    DEVOLVERÁ SIEMPRE UN RESULT<MATRIX, &STR>

*/

fn main() {
    let file_path:Vec<String> = env::args().collect();
    
    let file_path = file_path.get(1).expect("You must provide a file path!");

    let mat = Matrix::from(file_path);


    if let Ok(mut nice) = mat {
        
        let mut new_mat = nice.clone();
        new_mat.transpose();
        nice.add_row(2, 3, 4.5);

        let square = &(0.0005 * &new_mat) * &nice; 
        println!("{}",  square.pow(3).unwrap() );
        
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
