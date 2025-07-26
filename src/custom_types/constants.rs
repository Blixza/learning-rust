// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("This is {}", THRESHOLD);
    println!("This is {}", is_big(n));

    // Error! Cannot modify a `const`
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}