//Project Euler Question 13

//Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.

use std::io::{BufRead, BufReader};
use std::fs::File;

extern crate num;
use num::{BigInt, Zero};


fn main() {
    println!("Starting Euler 13 !");
    let mut answer : u32 = 0;
    let big_array = big_read();


    let mut answer : BigInt = Zero::zero();
    for zz in big_array.iter(){
        answer = answer + zz;

    }
    println!("answer is {}",answer);

    }



fn big_read() -> Vec<BigInt>  {

    let fileReader = BufReader::new(File::open("./src/number.txt").expect("open failed"));
      let mut vector: Vec<BigInt> = vec![];

       for line in fileReader.lines() {
           match line {
               Err(why)   => panic!("{:?}", why),
               Ok(string) => match string.trim().parse::<BigInt>(){
                   //ParseBigIntError => panic!("Not a number!"),
                   Err(why)   => panic!("{:?}", why),
                   Ok(x)=> vector.push(x)
               }
           }
       }
return vector;

}
