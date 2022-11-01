fn sum(x: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..x.len() {
        res += x[i]
    }
    res
}

fn sub(x: &[i32]) -> i32 {
    let mut val = 0;
    for i in 0..x.len() {
        val -= x[i]
    }
    val
}

fn main() {
    let arr = [10, 20, 30, 40, 15];
    // let res = sum(&arr);
    // println!("sum {}", res);
    let res = sub(&arr);
    println!("sub {}", res);
    print!("");
}


