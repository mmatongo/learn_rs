// the -> denotes the return type
fn add(x: i32, y: i32) -> i32 {
    // return and ; not needed here
    x + y
}


fn main() {
    /*
     * specifying the type is not necessary
     * unless you need a specific type return
     * NB: you cannot mix types
     */
    println!("1 + 2 = {}", 1u32 + 2);
    let res = add(10, 13);

    println!("10 + 13 = {:?}", res);

    // inline math is possible but this is weird
    print!("{} + {} = {}\n", 34, 80, 80 + 34);
    // this will be evaluated according to PEMDAS or BODMAS
    print!("{}", (23 - 6) % 5 + 20 * 30 / (3 + 4));
}
