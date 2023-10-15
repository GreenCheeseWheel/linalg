# 'LINALG' library

### What is it?
LINALG is a linear algebra library written in Rust, for Rust.
It's purpose is to be of general use, practical and fast.

### Main features
Right now it's main features are:
- A 'Matrix' struct
- Matrix instantiation through .csv files or any Vec\<f64\> collection
- Addition and Multiplication of matrices
- Calculation of the determinant of a matrix
- Row reduction by Gauss method (Echelon form of a matrix)
- Linear system of equations solver
- Elementary operations on any matrix
- Matrix multiplication by any scalar value
- Matrix transposing
- Matrix indexing using tuples
- Matrix formatting for CLI display
- .clone() method implemented for the Matrix struct
- Operator overloading for +, - and *

### What's to be implemented in the near future
There is still a lot left to do, but my main priorities are:
- Optimizing matrix multiplication and determinant calculations
- Adding Vector and Matrix structs that can be stack allocated
- Starting work on diagonalization (That is, calculating eigenvectors and eigenvalues)

### Where to find documentation
You won't find any external site for that right now, but you can find the basics right below this.





## LINALG basics

You can create a matrix using either the Matrix::new or the Matrix::from methods.
The first one takes two arguments, rows, and columns for your matrix.
The second one takes a string literal, and it's used to read a matrix from a csv file

```Rust
    // This creates a 2 by 3 null matrix
    let matrix = linalg::Matrix::new(2, 3); 

    // This creates a matrix based on the csv file path provided
    let matrix = linalg::Matrix::from("./matrix.csv");

    // This fills your matrix with the provided elements
    // If the amount of elements in your matrix don't match the length of the Vec
    // provided then it returns false, and doesn't change your matrix,
    // else it return true and now your matrix is all set.
    matrix.set_data(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    // This prints the recently created matrix like so:
    println!("{}", matrix);
    // R1 <-> 1.0  2.0  3.0
    // R2 <-> 4.0  5.0  6.0
```

If you have two matrices you can perform addition or multiplication with them like this

```Rust
    // This performs addition
    let result:Result<Matrix, &str> = &matrix_one + &matrix_two; 
    
    // This performs multiplication
    let result:Result<Matrix, &str> = &matrix_one * &matrix_two; 
```

Take into account that, when performing any operation, the dimensions of your matrices are taken into account. So pretty much any operation returns a Result enum with either your new matrix or an error string. You are then free to manage errors how you find fit.

If you need to solve a system of linear equations you might use the linalg::lineal_eq::solve_system function.

```Rust
    let matrix_with_inverse = Matrix::from("file_path.csv");
    
    let solution_vec:Matrix = solve_system(matrix_with_inverse, some_column_vector);
```

*ATTENTION*
The Matrix struct uses a single Vec collection to represent the instantiated matrix, so this makes indexing
a bit more difficult. This decision was taken in order to optimize memory usage.

So, if you want to get the element of the i-th row and j-th column you would index like this

```Rust
    let indexing_through_data = some_matrix.data[i * some_matrix.cols + j];

    let indexing_through_overloading = some_matrix[(i, j)]; // Notice we use a tuple inside the brackets
```