# 'LINALG' library

### What is it?
LINALG is a linear algebra library written in Rust, for Rust.
It's purpose is to be of general use, practical and fast.

### Main features
Right now it's main features are:
- A 'Matrix' struct
- Matrix instantiation through .csv files or any Vec\<f64\> collection
- Addition and Multiplication of matrices
- Calculation of the determinant of a square matrix
- Row reduction by Gauss method (Echelon form of a matrix)
- Row swapping
- Matrix transposing
- Matrix formatting for CLI display
- .clone() method implemented for the Matrix struct
- Operator overloading for + and *

### What's to be implemented in the near future
There is still a lot left to do, but my main priorities are:
- Implementing basic row operations (Scalar multiplication and row addition)
- Optimizing matrix multiplication and determinant calculations
- Implementing partial pivoting to reduce rounding errors
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