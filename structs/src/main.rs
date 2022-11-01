struct Person {
    age: i32,
    name: String,
    gender: String,
    height: f32,
}

fn main() {
    let daniel = Person {
        age: 24,
        name: String::from("Daniel"),
        gender: String::from("M"),
        height: 6.01
    };

    println!("{}", daniel.name);
}
