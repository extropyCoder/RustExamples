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

    //    println!("10! = {}", factorial(ToBigInt::to_bigint(&1234).unwrap()));
        }


fn factorial(n: i64) -> i64{

    if n == 1{
         return 1;
    }
    else{
        return n * factorial(n-1);
}

}


// pub fn factorial<T: Integer>(num: &T) -> T {
// 	if *num == Zero::zero() ||
// 		*num == One::one() {
// 		return One::one();
// 	} else {
//         let z = *num - One::one();
// 		return *num * factorial(&z);
// 	}
// }
