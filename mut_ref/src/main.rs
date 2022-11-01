fn modifies(x: &mut f64) {
    *x = 1.7;
}

fn main() {
    let mut res = 10.0;
    modifies(&mut res);
    println!("{}", res);
}
