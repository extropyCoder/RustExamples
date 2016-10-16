//Project Euler Question 22




//Using names.txt  a 46K text file containing over five-thousand first names,
//begin by sorting it into alphabetical order. Then working out the alphabetical value for each name,
//multiply this value by its alphabetical position in the list to obtain a name score.
//For example, when the list is sorted into alphabetical order,
//COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
//So, COLIN would obtain a score of 938 × 53 = 49714.
//What is the total of all the name scores in the file?


use std::io;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};


// file has been cleaned a little

    fn main() {
        println!("Starting Euler 22!");

        let mut word_vec = big_read();
        word_vec.sort();
        println!("{:?}",word_vec);
}

fn big_read() -> Vec<String>  {

    let fileReader = BufReader::new(File::open("./src/names.txt").expect("open failed"));
      let mut vector: Vec<String> = vec![];

       for line in fileReader.lines() {
           match line {
               Err(why)   => panic!("{:?}", why),
               Ok(string) => match string.trim().parse::<String>(){
                   Err(why)   => panic!("{:?}", why),
                   Ok(x)=> vector.push(x)
               }
           }
       }
return vector;

}
