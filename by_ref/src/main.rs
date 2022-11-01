// here we reference x
fn by_ref(x: &i32) -> i32{
    // here we dereference it : note the symbols '& for referencing' & '* for dereferencing'
    *x + 1
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&23);

    println!("{} : {}", res1, res2);
    // 11 : 24
}
