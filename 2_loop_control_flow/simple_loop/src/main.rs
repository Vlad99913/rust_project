fn main() {
    let mut x = 1;
    loop {
        x += 1;
        println!("{}", x);
        if x > 10 {
            break;
        }
    }
    println!("end of loop");
}
