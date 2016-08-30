//Project Euler Question 8


//The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
//Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    println!("Starting Euler 7 !");
    let mut largest : u64 = 0;

let array: [u64; 1000] = read_file();


for x in 0..987 {
    let slico = &array[x .. x+13];
    let sum_value = product_sequence(slico);
    if sum_value > largest {
        largest = sum_value;
    }

}

// let testio : [u64;5] = [1,2,3,4,5];
// largest  = product_sequence(&testio);
 println!("Largest product is {} " ,largest);

}

// fn sum_sequence(xs:&[u64])-> u64{
//     return xs.iter().fold(0, |sum, x| sum + x);
// }

fn product_sequence(xs:&[u64])-> u64{
    // for xx in xs{
    //     println!("{} " ,xx);
    // }
    let total_product = xs.iter().fold(1, |product, x| product * x);
    //println!("Total product is {} " ,total_product);
    return total_product;
}

fn read_file()->[u64;1000]{

    let f = BufReader::new(File::open("./src/input.txt").expect("open failed"));

    let mut xs: [u64; 1000] = [0; 1000];
    let mut position = 0;
    for line in f.lines() {
     for c in line.expect("lines failed").chars() {
             xs[position] = c.to_digit(10).expect("Invalid Character ! ") as u64;
             position +=1;

         }
     }

     return xs;
}
