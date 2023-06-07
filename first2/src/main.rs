
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut array:[[i32;3];3] = [[0; 3]; 3];
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            let temp = matrix[i][j];
            array[i][j] = matrix[j][i];
            array[j][i] = temp;
        }
    }
    
    return array;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in matrix{
        for index in 0..i.len(){
            if index==0 {
                print!("|{}",i[index]);
            }else if index == i.len()-1{
                println!(" {}|",i[index]);
            }else{
                print!(" {}",i[index]);
            }
        }
    }
  
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    // println!("{matrix:?}")
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}