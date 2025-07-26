fn main() {
    let shadowed_binding = 1;

    {
        println!("Before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = 2;
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("After inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 3;
    println!("shadowed in outer block: {}", shadowed_binding);
}