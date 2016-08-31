//Project Euler Question 9


//A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//a2 + b2 = c2
//For example, 32 + 42 = 9 + 16 = 25 = 52.
//There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//Find the product abc.



fn main() {
    println!("Starting Euler 9 !");
let mut stop_loop = false;
// brute force because I've forgotten the correct way to do this
for a in 1..999 {
    if stop_loop == true{
        break;
    }
for b in 1..999 {
    if stop_loop == true{
        break;
    }
    if a + b > 1000{
        break;
    }
for c in 1..999 {
    if a + b + c == 1000{
        if a*a +b*b == c*c{
        let product = a*b*c;
        println!("Foound ! {}, {}, {}" ,a,b,c);
        println!("Product is {}" ,product);
        stop_loop=true;
    }
    }


}
}
}


}
