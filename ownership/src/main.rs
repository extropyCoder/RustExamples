fn main() {
    println!("ownership examples!");

    // mutable reference
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);


}


fn foo(){
    let v = 1;

    let v2 = v;

    println!("v is: {}", v);
}
