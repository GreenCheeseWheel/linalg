use std::{thread, time::Duration, arch::x86_64::_mm256_add_ps, sync::{mpsc, Arc, Mutex}};

use linalg::{matrix::Matrix, lineal_eq::solve_system, thread_pool::ThreadPool, matrix_product::async_mat_prod};
/*
    REGLAS PARA EL DESARROLLO DE LA LIBRERIA

    1) LAS OPERACIONES MATRICIALES CON SOBRECARGA DE OPERADORES DEVOLVERAN
    EL RESULTADO ESPERADO O ENTRARÁN EN PANICO. JAMÁS DEVOLVERAN UN ENUM DE TIPO RESULT

    2) CUALQUIER FUNCIÓN QUE REALICE UNA OPERACIÓN ARITMÉTICA SOBRE UNA O MÁS MATRICES
    DEVOLVERÁ SIEMPRE UN RESULT<MATRIX, &STR>
*/

fn main() {
   
    let mat1 = Matrix::from("./matrix.csv").unwrap();
    let mat2 = mat1.clone();
    let (tx, rx) = mpsc::channel();
    
    async_mat_prod(&mat1, &mat2, Arc::new(Mutex::new(tx)));


    thread::sleep(Duration::from_secs(1));

    if let Ok(matrix) = rx.try_recv()
    {
        println!("{}", matrix);
    }
    
    /*
        TODO:

        IMPLEMENT CONCURRENCY

        IMPLEMENT DIAGONALIZATION IN THESE STEPS:
            1) Find the characteristic polynomial
            2) Find the roots of the polynomial
            3) Reduce the matrix by Gauss-Jordan and find the eigen-space
            4) Check the usual stuff
    */
}
