use std::fs;

// THIS FUNCTION READS .csv FILES AND RETURN A VEC 
// WITH IT'S VALUES
pub fn read_csv(file_path:&str) -> Option<Vec<f64>>
{
    let file = match fs::read(file_path){
        Ok(file_vec) => file_vec,
        Err(_) => vec![0]
    };

    if file.len() == 1
    {
        return None;
    }

    let mut result:Vec<f64> = vec![];
    let mut stored_ind = 0;

    for i in 1..file.len()
    {
        if file[i] == b','
        {
            let mut num = String::new();

            for byte in file[stored_ind..i].iter()
            {
                num.push(*byte as char);
            }
        
            let num:f64 = num.parse().expect("Invalid csv file provided");    
            result.push(num);

            stored_ind = i+1;
        }
    }


    Some(result)
}