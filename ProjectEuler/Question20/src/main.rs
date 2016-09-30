//Project Euler Question 20





//n! means n × (n − 1) × ... × 3 × 2 × 1
//For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//Find the sum of the digits in the number 100!




extern crate num;
use num::{Integer,BigInt, Zero,One};
use num::bigint::{ToBigInt};
    fn main() {
        println!("Starting Euler 20!");

        println!("10! = {}", factorial(&(10.to_bigint().unwrap())));
        }

//
// fn factorial(n: &BigInt) -> BigInt{
//     let z = One::one();
//     if *n == z{
//          return z;
//     }
//     else{
//         return *n * factorial(&(*n -z));
// }
//
// }


pub fn factorial<T: Integer>(num: &T) -> T {
	if *num == Zero::zero() ||
		*num == One::one() {
		return One::one();
	} else {
		return *num * factorial(&(*num - One::one()));
	}
}
