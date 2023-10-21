use std::{thread, time::{Duration, Instant}, arch::x86_64::_mm256_add_ps, sync::{mpsc, Arc, Mutex}};

use linalg::{matrix::Matrix, lineal_eq::solve_system, thread_pool::ThreadPool, matrix_product::{async_mat_prod, self}};
/*
    REGLAS PARA EL DESARROLLO DE LA LIBRERIA

    1) LAS OPERACIONES MATRICIALES CON SOBRECARGA DE OPERADORES DEVOLVERAN
    EL RESULTADO ESPERADO O ENTRARÁN EN PANICO. JAMÁS DEVOLVERAN UN ENUM DE TIPO RESULT

    2) CUALQUIER FUNCIÓN QUE REALICE UNA OPERACIÓN ARITMÉTICA SOBRE UNA O MÁS MATRICES
    DEVOLVERÁ SIEMPRE UN RESULT<MATRIX, &STR>
*/

fn main() {
    

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
