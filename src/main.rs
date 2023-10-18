use linalg::{matrix::Matrix, lineal_eq::solve_system};
/*
    REGLAS PARA EL DESARROLLO DE LA LIBRERIA

    1) LAS OPERACIONES MATRICIALES CON SOBRECARGA DE OPERADORES DEVOLVERAN
    EL RESULTADO ESPERADO O ENTRARÁN EN PANICO. JAMÁS DEVOLVERAN UN ENUM DE TIPO RESULT

    2) CUALQUIER FUNCIÓN QUE REALICE UNA OPERACIÓN ARITMÉTICA SOBRE UNA O MÁS MATRICES
    DEVOLVERÁ SIEMPRE UN RESULT<MATRIX, &STR>
*/

fn main() {
    // We work in a matrix.csv file that represents a non-singular matrix
    let file_result = Matrix::from("./matrix.csv");

    if let Ok(matrix) = file_result {
        
        let echelon = matrix.get_echelon();
        println!("Number of row swaps needed to get echelon form: {}", echelon.1);

        // This matrix has an inverse, so we unwrap the result
        let inverse = matrix.get_inverse().unwrap();
        
        let solution_vec = solve_system(&inverse, &Matrix::from_iterator(inverse.rows, 1, vec![1.0; inverse.rows]));
        
        println!("SOLUTION: {}", solution_vec.as_ref().unwrap());

        println!("ORIGINAL VEC: {}", &inverse * &solution_vec.unwrap());

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
