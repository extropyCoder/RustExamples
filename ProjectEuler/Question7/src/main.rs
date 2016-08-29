//Project Euler Question 7


//The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
//Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    println!("Starting Euler 7 !");
    let mut largest = 0;
    let nums_array = read_file();

//    let mut iter = vec_nums.iter().enumerate();
//.fold(0, |sum, x| sum + x);

//        println!("Number is  {}", &vec_nums[ii]);
//        if i+12 > &vec_nums.length(){
//            break;
//        }


// for chunk in vec_nums.chunks(13) {
//     println!("{:?}", chunk);
// }

let array: [u32; 1000] = read_file();


for x in array.iter() {

    print!("{} ", x);
}

println!("Largest product is {} " ,largest);

}




fn read_file()->[u32;1000]{

    let f = BufReader::new(File::open("./src/input.txt").expect("open failed"));

    let mut xs: [u32; 1000] = [0; 1000];
    let mut position = 0;
    for line in f.lines() {
     for c in line.expect("lines failed").chars() {
             xs[position] = c.to_digit(10).expect("Invalid Character ! ");
             position +=1;

         }
     }

     return xs;
}



// fn read_file()-> Vec<u32>{
//
//     let f = BufReader::new(File::open("./src/input.txt").expect("open failed"));
//     let mut v: Vec<u32> = vec![];
//     for line in f.lines() {
//          for c in line.expect("lines failed").chars() {
//              v.push(c.to_digit(10).expect("Invalid Character ! "));
//              //println!("Character: {}", c);
//          }
//      }
//
//      return v;
// }
