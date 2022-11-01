fn main() {
    let arr = [10, 20, 30, 40, 50];
    let arr_first = arr[0];
    println!("first {}", arr_first);

    for i in 0..5 {
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len());
}
