fn modifiable(name: &String::new) -> String {
    *name = "Daniel"
}

fn main() {
    // get input from std input
    let mut name = "John";
    modifiable(&mut name);

    println!("{}", name);
}
