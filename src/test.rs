fn main() {
    let start: u32 = 0;
    let end: u32 = 5;

    for i in start..end {
        println!("{}: Hello From Rust", i)
    }

    let a: u32 = 1;
    let b = 2;
    println!("a + b = {} + {} = {}", a, b, a + b);

    let _x = 2;
    for x in start..end {
        println!("2**{} = {}", x, 2_u32.pow(x))
    }
}
