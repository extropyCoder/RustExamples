//Project Euler Question 2
//Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
//1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.



fn main() {
    println!("Starting Euler 2 !");
//    testAndAdd(0);
let mut sum = 0;
for x in 1..40 {

        let y = fib(x);
        if y > 4000000 {
            break;
        }
        else{
            if y % 2 == 0{
                sum += fib(x);
            }
        }

    }
println!("Sum is {}",sum);

}

fn fib(x: u32) -> u32 { if x == 1 {1} else if x == 2 {2} else {fib(x-1) + fib(x-2)} }
