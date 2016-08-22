//Project Euler Question 4
//A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//Find the largest palindrome made from the product of two 3-digit numbers.


fn main() {
    println!("Starting Euler 4 !");
//let mut stop_loop = false;
let mut largest = 0;
for x in 1..900{
  // if stop_loop == true{
  // break;
  // }
  for y in 1..900{
    let z = (1000-x)*(1000-y);
    if is_palindrome(z)==true{
        if z > largest{
            largest  = z;
        }
    //    println!("largest is  {}",z);

    }
  }
}

println!("largest is  {}",largest);


}

fn is_palindrome(n: u32) -> bool {
  let x = n.to_string();
  let y = x.as_bytes();
  // println!(" n is  {}",n);
  let length = y.len();
  for ii in 0..((length)/2) {
    if y[ii] != y[length-ii -1] {
      return false;
    }
  }
//println!(" palindrome found :   {}",n);
return true;
}
