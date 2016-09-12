//Project Euler Question 14



//The following iterative sequence is defined for the set of positive integers:
//n → n/2 (n is even)
//n → 3n + 1 (n is odd)

//Using the rule above and starting with 13, we generate the following sequence:
//13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//Which starting number, under one million, produces the longest chain?
//NOTE: Once the chain starts the terms are allowed to go above one million.



fn main() {
    println!("Starting Euler 14 !");
    let mut answer : ( u32,u32) = (0,0);
    for n in 1 .. 1000000 {
        let latest = calc_collatz_length(n);
        if latest > answer.1 {
            answer.0 = n;
            answer.1 = latest;
        }
    }

    println!("n is {}, length {}",answer.0,answer.1);

    }

fn calc_collatz_length(n:u32) -> u32{
    let (n,length) = get_next_collatz((n as u64,1));
    return length;
}

fn get_next_collatz(input : (u64,u32)) -> (u64,u32){
    //n → n/2 (n is even)
    //n → 3n + 1 (n is odd)
    if input.0 ==1{
        return input;
    }
    else if input.0 % 2 == 0 {
            let new_n = input.0 /2;
            let new_length = input.1 + 1;
            return get_next_collatz((new_n,new_length));
    }
    else {
        let new_n = 1 + input.0 * 3;
        let new_length = input.1 + 1;
        return get_next_collatz((new_n,new_length));

    }

}
