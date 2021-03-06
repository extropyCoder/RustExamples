//Project Euler Question 19




//You are given the following information, but you may prefer to do some research for yourself.

//    1 Jan 1900 was a Monday.
//    Thirty days has September,
//    April, June and November.
//    All the rest have thirty-one,
//    Saving February alone,
//    Which has twenty-eight, rain or shine.
//    And on leap years, twenty-nine.
//    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

//How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?



    fn main() {
        println!("Starting Euler 19!");
        println!("d={}",day_of_week(26,03,1972));

        let mut total = 0;
        for yy in 1901 .. 2000 {
            for mm in 1 ..13{
                if day_of_week(1,mm,yy)==0{
                    println!("date ={} / {}",mm,yy);
                        total+=1;
                    }
                }
            }
        println!("Answer is {}",total);
}

fn day_of_week(dd :i16,mm : i16, yy : i16) -> i32 {
// use Gaussian function
//   w = (d+floor(2.6*m-0.2)+y+floor(y/4)+floor(c/4)-2*c) mod 7

//   Y = year - 1 for January or February
//   Y = year for other months
//   d = day (1 to 31)
//   m = shifted month (March = 1, February = 12)
//   y = last two digits of Y
//   c = first two digits of Y
//   w = day of week (Sunday = 0, Saturday = 6)


let m = (mm - 3) % 12 + 1;

let mut Y = yy;

if m > 10 {
    Y = yy - 1;
}


let y :f32   = (Y % 100) as f32;
let c :f32 = ((Y - (Y % 100)) / 100) as f32;

let mf = (2.6* m as f32 -0.2).floor();


let w1 :i32 = ((dd as f32 + mf + y + (y/4.0) + c/4.0 -2.0*c)) as i32;
let w2 = w1 % 7;
return w2;


}
