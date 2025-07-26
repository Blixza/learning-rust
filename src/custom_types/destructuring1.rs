fn main() {
    let coordinates = (3i32, 4i32);

    // Without destructiring
    // let x: i32 = coodrinates.0;
    // let y: i32 = coordinates.1;
    // println!("x: {}, y: {}", x, y);

    // With destructuring
    let (x, y) = coordinates;
    println!("x: {}, y: {}", x, y);
}