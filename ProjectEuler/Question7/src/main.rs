//Project Euler Question 7


//The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
//Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    println!("Starting Euler 7 !");
    let x = 0;
    let vec_nums =  read_file();

    for i in &vec_nums {
        println!("Number is  {}", i);
    }

println!("Largest product is {} " ,x);

}

fn read_file()-> Vec<u32>{

    let f = BufReader::new(File::open("./src/input.txt").expect("open failed"));
    let mut v: Vec<u32> = vec![];
    for line in f.lines() {
         for c in line.expect("lines failed").chars() {
             v.push(c.to_digit(10).expect("Invalid Character ! "));
             //println!("Character: {}", c);
         }
     }

     return v;
}
