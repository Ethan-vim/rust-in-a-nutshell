fn main() {
    let lower_limit: u8 = 0; // Inclusive
    let upper_limit: u8 = 10; // Inclusive or Exclusive based from syntax of for loop

    for v in lower_limit..upper_limit { // Exclusive
        println!("Exclusive iterations: {}", v);
    }
    println!("");
    for v in lower_limit..=upper_limit { // Inclusive
        println!("Inclusive iterations: {}", v);
    }
}
