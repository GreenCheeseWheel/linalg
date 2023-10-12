use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

// THIS FUNCTION READS .csv FILES AND RETURN A VEC 
// WITH IT'S VALUES
pub fn read_csv(file_path:&str) -> Option<Vec<f64>>
{
    let file = read_lines(file_path);



    if file.len() == 0
    {
        return None;
    }

    let mut result:Vec<f64> = vec![];
    

    for line_num in 0..file.len()
    {
        let mut stored_ind = 0;

        let line_bytes = &file[line_num];

        for (col, byte) in line_bytes.as_bytes().iter().enumerate()
        {
            if *byte == b','
            {
                let matrix_element:f64 = line_bytes[stored_ind..col].parse().expect(&format!("Element in row {line_num} and column {col} is not a number"));
                result.push(matrix_element);
                stored_ind = col+1;
                continue;
            }

            if col == line_bytes.len()-1
            {
                let matrix_element:f64 = line_bytes[stored_ind..].parse().expect(&format!("Element in row {line_num} and column {col} is not a number"));
                result.push(matrix_element);
            }

        }

    }



    Some(result)
}