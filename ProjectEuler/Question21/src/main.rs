//Project Euler Question 21






//Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
//If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
//For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//Evaluate the sum of all the amicable numbers under 10000.


    fn main() {
        println!("Starting Euler 21!");


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
        let sum:i32 = 0;



        return sum;
}
