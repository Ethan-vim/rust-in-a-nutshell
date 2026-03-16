fn main() {
    let original: u8 = 67;
    let transfered = Box::new(original);
    println!("The number is: {}", transfered);
}
