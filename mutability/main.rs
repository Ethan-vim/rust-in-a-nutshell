fn main() {
    let x: u8 = 5; // Immutable;  Cannot be changed
    //  x = 10 This will throw a error, because x is immutable
    println!("x: {}", x);
    let mut y: u8 = 10; // Mutable; Can be changed
    // y = 11 This is fine because y is mutable
    y = 11;
    println!("y: {}", y);
}
