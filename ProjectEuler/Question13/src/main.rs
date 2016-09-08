//Project Euler Question 13

//Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    println!("Starting Euler 13 !");
    read_file();


    }

    fn read_file()-> [[u32; 50];50] {

        let f = BufReader::new(File::open("./src/numbers.txt").expect("open failed"));
        let mut xs: [[u32; 50];50] =[[0;50];50];
        //let mut xs: [u64; 1000] = [0; 1000];
        let mut position = 0;
        let mut line_number = 0;
        for line in f.lines() {
         for c in line.expect("lines failed").chars() {
                 xs[position][line_number] = c.to_digit(10).expect("Invalid Character ! ") as u32;
                 position +=1;


             }
             line_number+=1;
         }

         return xs;
    }
