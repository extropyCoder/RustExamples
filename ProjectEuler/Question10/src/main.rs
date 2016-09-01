//Project Euler Question 10

//The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//Find the sum of all the primes below two million.


fn main() {
    println!("Starting Euler 10 !");

    let mut total_sum = 0;
    let mut counter= 2;
    while counter < 2000000{
        if counter==2||counter==3 {
            total_sum = total_sum + counter;
        }
        else if possible_prime(counter)==true{
            if is_prime(counter)==true{
            total_sum = total_sum + counter;
            //println!("Total Sum is now {}",total_sum);

            }
        }
        counter = counter +1;
    }
    println!("Total Sum is {}",total_sum);

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
    for x in 2..limit + 1 {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
