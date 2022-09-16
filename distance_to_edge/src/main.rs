
 use std::io;
 use std::cmp;

 fn main() {
     // get standard input stream
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read the line");

     //Feeds input to a vector.
     let rec_amount : Vec<u32> = input.trim().split(" ").map(|input| input.parse().expect("Failed to parse input")).collect();

     print_answer(rec_amount[0] - 1, rec_amount[1] - 1);
 }
//  Recieves rows and columns and prints the answer
 fn print_answer(rows: u32, columns: u32){
    for row in 0..(rows + 1){
        for column in 0..(columns + 1) {
            print!("{}", check_num(row, rows, column, columns));
        }
        println!();
    }
 }
// Recieves row and column and returns the distance to the nearest "edge" of the given rectangle as a string
 fn check_num(row_nr: u32, rows: u32, column_nr: u32, columns: u32) -> String{
    let row_distance: u32;
    let column_distance: u32;
    if (rows - row_nr) < row_nr {
        row_distance = rows - row_nr;
    }
    else {
        row_distance = row_nr;
    }
    if (columns - column_nr) > column_nr {
        column_distance = column_nr;
    }
    else {
        column_distance = columns - column_nr;
    }

    if cmp::min(column_distance, row_distance) >= 9 {
        return ".".to_string();
    }
    else {
        return (cmp::min(column_distance, row_distance)+1).to_string();
    }
    
 }