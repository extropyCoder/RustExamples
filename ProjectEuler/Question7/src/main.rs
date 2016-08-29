//Project Euler Question 7

//By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//What is the 10001st prime number?




fn main() {
    println!("Starting Euler 7 !");
    let mut counter = 6;    //start at 13
    let mut x= 13;
    while counter < 10002{
        if possible_prime(x)==true{
            if is_prime(x)==true{
                if (counter % 100 == 0) || (counter > 10000) {
                    println!("The {} Prime number is {}",counter,x);
                }
                counter = counter +1;

            }
        }
        x = x+1;
    }



}

fn possible_prime(x: u64) -> bool {
     if ((x -1) % 6 == 0)||((x +1) % 6 == 0){
         true
     }
      else {
          false
      }
  }

fn is_prime(n: u64) -> bool {
    let limit:u64 = ((n as f64).sqrt()).ceil() as u64;
    for x in 2..limit +1 {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
