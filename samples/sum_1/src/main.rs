fn main() {
    // for iterations make your first assignment of an 
    // iterator mutable. by default things are not mutable
    let mut sum = 0;
    for i in 0..10 {
        sum += i;
    }
    println!("{}", sum);
}
