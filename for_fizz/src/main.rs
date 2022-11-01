fn main() {
    for i in 3..30 {
        if i % 3 == 0 {
            println!("Fizz {}", i);
        } else if i % 5 == 0 {
            println!("Buzz {}", i);
        }
    }
}
