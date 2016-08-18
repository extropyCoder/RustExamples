//Project Euler Question 3
//The prime factors of 13195 are 5, 7, 13 and 29.
//What is the largest prime factor of the number 600851475143 ?



fn main() {
    println!("Starting Euler 3 !");

//let input:f64 = 600851475143.0;
let input:u64 = 600851475143;
let limit:u64 = ((input as f64).sqrt()).ceil() as u64;

println!(" limit is  {}",limit);
let mut largest = 1;

for x in 1..limit {

    let p1 = 6 * x - 1;
    let p2 = 6 * x + 1;
    if p1 > limit {
        break;
    }
    if input % p1 ==0 {
        let y = input / p1 ;
        if is_prime(p1) ==true {
         if p1 > largest{largest = p1;}
        }
        if is_prime(y) ==true{
            if y > largest{largest = y;}
        }
    }
    else if input % p2 ==0 {
        let y = input / x ;
        if is_prime(p2) ==true {
         if p2 > largest{largest = p2;}
        }
        if is_prime(y) ==true{
            if y > largest{largest = y;}
        }
    }

}


println!("largest is  {}",largest);


}

//fn possible_prime(x: u64) -> bool { if ((x -1) % 6 == 0)||((x +1) % 6 == 0) || (x==2) || (x==3)    {true} else {false}}
fn is_prime(n: u64) -> bool {
    let limit:u64 = ((n as f64).sqrt()).ceil() as u64;
    for x in 2..limit {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
