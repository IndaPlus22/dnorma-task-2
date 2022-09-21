use std::io;
use std::io::prelude::*;

fn main() {
    
    let input = io::stdin();
    let lines = input.lock().lines().map(|_line| _line.ok().unwrap()).collect::<Vec<String>>();

    for num in lines.iter() {
        let numbers : Vec<i64> = num.split(" ").map(|input| input.parse::<i64>().unwrap()).collect();

        println!("{}", (numbers[0] - numbers[1]).abs());
    }
}