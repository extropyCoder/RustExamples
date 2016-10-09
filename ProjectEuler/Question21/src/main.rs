//Project Euler Question 21






//Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
//If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
//For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
// The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//Evaluate the sum of all the amicable numbers under 10000.

use std::collections::HashMap;


    fn main() {
        println!("Starting Euler 21!");
        let mut sum = 0;
        let mut pairs = HashMap::<i32, Vec<i32>>::new();
        for n in 1 .. 10000{
            let x:i32= get_sum_divisors(n);


            if !pairs.contains_key(&x) {
                let mut vec =Vec::new();
                vec.push(n);
                pairs.insert(x,vec);
            }
            else{
                let mut vec = pairs.get_mut(&x).unwrap();
                vec.push(n);
            }
        }
        // now we have a map, so go through and add where there are pairs
        for (x, vec_of_amicables) in &pairs {
            if vec_of_amicables.len() == 2{
                println!("x n is {} {:?}", x, vec_of_amicables);
                for vals in vec_of_amicables.iter(){
                    println!("adding {} ", vals);
                    sum = sum + vals;
                }
            }

        }

        println!("Sum is {}",sum);
        }


fn get_sum_divisors(n:i32) -> i32{

    return sum_vector(find_divisors(n));
}

fn find_divisors(n:i32)->Vec<i32> {

    let mut new_vector: Vec<i32> = Vec::new();
    new_vector.push(1);

    let limit:i32 = ((n as f64).sqrt()).ceil() as i32;
     for x in 2..limit {
         if n % x == 0 {
             new_vector.push(x);
             let other = n / x;
             new_vector.push(other);
         }
     }




    return new_vector;
}

fn sum_vector(v:Vec<i32>) ->i32{
        return v.iter().fold(0, |sum, i| sum + i);
}
