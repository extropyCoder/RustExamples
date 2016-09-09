//Project Euler Question 13

//Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.

use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    println!("Starting Euler 13 !");
    let mut answer : u32 = 0;
    let big_array = read_file();

let new_array = reduce_array(&big_array);

    for zz in big_array.iter(){
        println!("zz is {?:}",zz);
    }

    // for x in new_array.iter(){
    //     println!("x is {:?}",x);
    // }



    }

    fn read_file()-> [[u32; 50];100] {

        let f = BufReader::new(File::open("./src/number.txt").expect("open failed"));
        let mut xs: [[u32; 50];100] =[[0;50];100];
        //let mut xs: [u64; 1000] = [0; 1000];
        let mut position = 0;
        let mut line_number = 0;
        for line in f.lines() {
         for c in line.expect("lines failed").chars() {
                //println!("reading line {}",line_number);
                //println!("reading position {}",position);
                 xs[position][line_number] = c.to_digit(10).expect("Invalid Character ! ") as u32;
                 position +=1;
             }
             line_number+=1;
             position = 0;
             if line_number >=50{
                 break;
             }
         }

         return xs;
    }

    fn reduce_array(input : &[[u32; 50];100]) -> [[u32; 12];100] {
        let mut output: [[u32; 12];100] =[[0;12];100];
        let mut num = 0;
        for x in input.iter() {
            //println!("num is {}",num);
            for y in 0 .. 11 {
                output[num][y] = x[y];
            }
            num+=1;
        }
        return output;

    }
