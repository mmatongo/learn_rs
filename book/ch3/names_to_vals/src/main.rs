fn main() {
    // we assign a value like so 
    let age = 24;
    let name = "Daniel";
    println!("name: {} - age: {}", name, age);

    // you can specify the type if you wish
    let age: u32 = 25;
    let name: &str = "Dee"; 
    println!("name: {} - age: {}", name, age);

    /* NOTE:
     * In the above examples i reassign the var's
     * however, it needs to be made clear that `let`
     * creates an entirely new variable if i do this
     * this is known as shadowing, I am unable to use the
     * first declaration as it does not exist anymore(or something like that)
     */

    let mut height: f32 = 6.01;
    println!("I am {} ft tall", height);
    height = 6.02;
    println!("sometimes I am {} ft tall", height);

    /*
     * completely forgot that this is a thing
     */
    let me: &str;
    me = "Daniel";
    println!("{}", me);
}
