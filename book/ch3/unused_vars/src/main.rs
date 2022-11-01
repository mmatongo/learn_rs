fn main() {
    /* 
     * not much here
     * just add a _
     */

    let name: &str = "Daniel";
    // this line will cause a compiler warning
    let age: u8 = 23;
    // this one will not
    let _age: u8 = 23;
    println!("{}", name);
    // _ is not a valid identifier
    // let _; will not work
}
