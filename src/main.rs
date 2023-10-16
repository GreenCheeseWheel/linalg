use linalg::matrix::Matrix;

/*
    REGLAS PARA EL DESARROLLO DE LA LIBRERIA

    1) LAS OPERACIONES MATRICIALES CON SOBRECARGA DE OPERADORES DEVOLVERAN
    EL RESULTADO ESPERADO O ENTRARÁN EN PANICO. JAMÁS DEVOLVERAN UN ENUM DE TIPO RESULT

    2) CUALQUIER FUNCIÓN QUE REALICE UNA OPERACIÓN ARITMÉTICA SOBRE UNA O MÁS MATRICES
    DEVOLVERÁ SIEMPRE UN RESULT<MATRIX, &STR>
*/

fn main() {

    let mat = Matrix::from("./matrix.csv").unwrap();

    let eigen = mat.qr_decompose();

   

    println!("{}", &eigen.0 * &eigen.1);

    /* 
        TODO:
       
        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
    
}  
