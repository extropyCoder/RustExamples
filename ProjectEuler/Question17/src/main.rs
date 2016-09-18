//Project Euler Question 17


//If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
//NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.



    fn main() {
        println!("Starting Euler 17!");




        }


    fn get_letter_count_tens(n:u16)->u16{
        match n {
            20 ... 29  => return 6,
            30 ... 39  =>return 6,
            40 ... 49  => return 5,
            50 ... 59  => return 5,
            60 ... 69  => return 5,
            70 ... 79  => return 7,
            80 ... 89  => return 6,
            90 ... 99  => return 6,
            _ => return 0,
        }

    }

fn get_letter_count_teen(n:u16)->u16{
    match n {
        10 => return 3,
        11 | 12  =>return 6,
        15 | 16 => return 7,
        13 | 14 | 18 | 19 =>return 8,
        17   =>return 9,
        _ => return 0,
    }

}

fn get_letter_count_unit(n:u16)->u16{
    match n {
        1 | 2 | 6 => return 3,
        4 | 5 | 9  =>return 4,
        3 | 7 | 8  =>return 5,
        _ => return 0,
    }
}
