fn main() {
    let a: i32 = 1_000_000;
    let b: i32 = 1000000;

    println!("a: {}, b: {}", a, b);

    if a == b {
        println!("Success!");
    }
}