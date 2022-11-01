#[derive(Debug)]
enum Species {
    Human,
    Dog,
}

#[derive(Debug)]
struct Person {
    species: Species,
    name: String,
    arms: i32,
}

fn main() {
    let daniel = Person {
        species: Species::Human,
        name: String::from("Daniel"),
        arms: 2,
    };

    match daniel.species {
        Species::Human => println!("{} is Human", daniel.name),
        Species::Dog => println!("{} is a Dog", daniel.name),
    }
}
