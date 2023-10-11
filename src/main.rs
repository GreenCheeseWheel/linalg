use linalg::{Matrix, matrix_product};

fn main() {
    let mut mat = Matrix::new(3, 3);
    let mut mat2 = Matrix::new(3, 3);

    mat.set_data(vec![1.0, 2.0, 5.0, 6.0, 2.0, 3.0, 0.5, 2.3, 6.0,]);
    mat2.set_data(vec![1.0, 2.0, 5.0, 6.0, 2.0, 3.0, 0.5, 2.3, 6.0,]);
    
    let mult = mat * mat2;

    match mult {
        Ok(res) => println!("{:?}", res.data),
        Err(_) => println!("ERROR")
    } 

}  
