//Project Euler Question 16

//2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//What is the sum of the digits of the number 2^1000?

extern crate num;
    use num::{BigInt, Zero};
    use num::bigint::ToBigInt;

    fn main() {
        println!("Starting Euler 16!");
        let n = 2.to_bigint().unwrap();
        let x = num::pow(n, 1000);
        //println!("answer is {}",answer);
        let nz = 123.to_bigint().unwrap();
        println!("answer is {}",get_digit_sum(nz));


        }



fn get_digit_sum(n:BigInt) -> BigInt {
    let ten = 10.to_bigint().unwrap();
    let mut answer = n;
    if n > ten {
        let n2 = n / ten;
        let answer = n %  10.to_bigint().unwrap() + get_digit_sum(n2) ;
    }
    return answer;
}
