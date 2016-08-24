//Project Euler Question 6

//The sum of the squares of the first ten natural numbers is,
//12 + 22 + ... + 102 = 385
//The square of the sum of the first ten natural numbers is,
//(1 + 2 + ... + 10)2 = 552 = 3025
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

// sum of squares is a pyramidal number
// = n(n+1)(2n+1)/6
// so for n = 100 = 338350

// and sum of 1..n = n(n+1)/2



fn main() {
    println!("Starting Euler 6 !");
    let x = sum_of_squares(100);
    let y = sum_to_n_squared(100);

println!("Sum of Squares is {} ",x);
println!("Square of sum is {} ",y);
println!("Difference is {} ",y - x);

}

fn sum_of_squares(n: u32) -> u32{
    return n*(n+1)*(2*n+1)/6;
}

fn sum_to_n_squared(n: u32) -> u32{
    let x =n*(n+1)/2;
    return x*x;
}
