//Project Euler Question 1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.


fn main() {
    println!("Starting Euler 1 !");
//    testAndAdd(0);
let mut sum = 0;
for x in 0..1000 {
    if x % 5 == 0 || x % 3 ==0 {
        sum += x;
    }
}
println!("Sum is {}",sum);

}

// Don't know how to do recursion yet

// fn testAndAdd(current : i32){
//     if current<=1000 {
//         if current % 5 == 0 || current % 5 ==0 {
//             return current + (testAndAdd(current + 1));
//         }
//         else{
//             return testAndAdd(current + 1);
//         }
//     }
// }
