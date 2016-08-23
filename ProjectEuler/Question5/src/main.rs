//Project Euler Question 5
//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

// brute force approach

fn main() {
    println!("Starting Euler 5 !");

let mut x: u64 = 2520;

while x < 9999999999{

    if is_divisible(x)==true{
        println!("smallest is  {}",x);

        break;
    }
    x=x+1;
}


}

fn is_divisible(n: u64) -> bool {
    for x in 2..20 {
        if n % x !=0 {
            return false;
        }
    }
return true;
}
