fn main() {
    let x = 1.0;
    let y = 2.0;
    let z = x + y;
    // note that numbers after the decimal if less than 0 are removed
    println!("{} + {} = {}", x, y, z as f32);

    let a: f32 = 1.3;
    let b: f32 = 1.4;
    let c = a + b;
    println!("the answer is {}", c); 
    //2.6999998 if returned as is
    //2.6999998092651367 if returned as f64
    //if we change the types of the variables to f64 we get 2.7

    print!("{}", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));
}
