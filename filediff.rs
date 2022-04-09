//program that finds the difference between two files
//and sends that differences into a of the name specified
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::env;

//helper function 
fn print_matrix(matrix: &Vec<Vec<EditCell>>) {
    for i in 0..matrix.len() {
        for b in 0..matrix[0].len() {
            print!("{}, {:?};", matrix[i][b].val, matrix[i][b].prev);
        }
        print!("\n");
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Down, //this means that this line is an addition
    Right, //this means that this line is an deletion
    DownRight, //this means that this file is an update or is equal, (depending on the value) 
    NoDirection, //nothing here
}

#[derive(Debug, Clone)]
pub struct EditCell {
    val: i32,
    prev: Direction, 
}

pub fn memo_min(memo: &Vec<Vec<EditCell>>, row: &usize, col: &usize) -> (i32, Direction) {
    let right = memo[*row][*col+1].val;
    let down_right = memo[*row+1][*col+1].val;
    let down = memo[*row+1][*col].val;

    if right < down_right && right < down {
        (right, Direction::Right)
    }else if down_right < down {
        (down_right, Direction::DownRight)
    }else {
        (down, Direction::Down)
    }
}

pub fn edit_distance(file_a: &Vec<String>, file_b: &Vec<String>) -> Vec<Vec<EditCell>> {
    //create the dynamic programming matrix
    let mut memo: Vec<Vec<EditCell>> = Vec::new();
    for i in 0..(file_a.len() + 1){
        memo.push(vec![EditCell{val: 0, prev: Direction::NoDirection, };file_b.len()+1]);
    }
    for i in 0..file_a.len() { memo[i][file_b.len()].val = file_a.len() as i32 - i as i32; }
    for i in 0..file_b.len() { memo[file_a.len()][i].val = file_b.len() as i32 - i as i32; }
    
    //run the edit distance algorithm
    for row in (0..file_a.len()).rev() {
        for col in (0..file_b.len()).rev() {
            if file_a[row].eq(&file_b[col]) {
                //the two file lines are equal
                memo[row][col].val = memo[row+1][col+1].val;
                memo[row][col].prev = Direction::DownRight;
            }else{
                let (min_value, direction) = memo_min(&memo, &row, &col);
                //thw two file lines are not equal
                memo[row][col].val = min_value + 1;
                memo[row][col].prev = direction;
            }
        }
    }
    //print_matrix(&memo);
    
    //now reconstruct the difference with the memo matrix 
    memo
}

fn process_edit_distance_to_file(matrix: &Vec<Vec<EditCell>>, file_a: &Vec<String>, file_b: &Vec<String>) -> Vec<String> {
    //process the edit distance information to a file 
    let (mut actual_row, mut actual_col) = (0, 0);
    let mut result: Vec<String> = Vec::new();
    while actual_row < file_a.len() && actual_col < file_b.len() {
        match matrix[actual_row][actual_col].prev {
            Direction::Down => {
                //this line is an addition
                result.push(String::from("-----DELETED LINE-----"));
                result.push(format!("File 1: {}", file_a[actual_row].clone()));
                result.push(String::from("---------------"));
                actual_row += 1;
            },
            Direction::DownRight => {
                //this is an update or a deletion
                if matrix[actual_row][actual_col].val > matrix[actual_row + 1][actual_col + 1].val {
                    //update
                    result.push(String::from("-----UPDATED LINE-----"));
                    result.push(format!("File 1: {}", file_a[actual_row].clone()));
                    result.push(format!("File 2: {}", file_b[actual_col].clone()));
                    result.push(String::from("---------------"));
                }else {
                    //equal
                    result.push(format!("File 1: {}", file_a[actual_row].clone()));
                }
                actual_row += 1;
                actual_col += 1;
            },
            Direction::Right => {
                //this is a deletion
                result.push(String::from("-----ADDED LINE-----"));
                result.push(format!("File 2: {}", file_b[actual_col].clone()));
                result.push(String::from("---------------"));
                actual_col += 1;
            },
            Direction::NoDirection => break
        }
    }
    //TODO: Why wee need this ? How to handle this edge cases
    //println!("ROW: {}, COL: {}, LEN_A: {}, LEN_B: {}", actual_row, actual_col, file_a.len(), file_b.len());
    while actual_row < file_a.len() {
        result.push(String::from("-----DELETED LINE-----"));
        result.push(format!("File 1: {}", file_a[actual_row]));
        result.push(String::from("-----------------"));
        actual_row += 1;
    }
    while actual_col < file_b.len() {
        result.push(String::from("-----ADDED LINE-----"));
        result.push(format!("File 2: {}", file_b[actual_col]));
        result.push(String::from("-----------------"));
        actual_col += 1;
    }
    result
}

fn main(){

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid arguments, expected two files");
    }
    let path1 = Path::new(&args[1]);
    let path2 = Path::new(&args[2]);
    
    //get the two files
    let file1 = File::open(path1).expect("Unable to open file 1");
    let file2 = File::open(path2).expect("Unable to open file 2");
    
    //create the two readers, one for each file
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    
    //create arrays with the content of the two files
    let lines1: Vec<String> = reader1.lines().map(|line| { line.unwrap() }).collect();
    let lines2: Vec<String> = reader2.lines().map(|line| {line.unwrap() }).collect();
    let edit_distance_matrix = edit_distance(&lines1, &lines2);
    let processed_result = process_edit_distance_to_file(&edit_distance_matrix, &lines1, &lines2);
    for line in processed_result {
        println!("{}", line);
    }
}





