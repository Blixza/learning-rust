// An attribute to hide warnings for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create a struct with field shorthand
    let name = String::from("Peter");
    let age = 27u8;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2f32, y: 0.4f32 };
    let another_point: Point = Point { x: 10.3f32, y: 0.2f32 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // the one
    let bottom_right = Point { x: 10.3f32, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    
    let rect1 = Rectangle {
        top_left: Point { x: 1.0, y: 4.0 },
        bottom_right: Point { x: 4.0, y: 0.0 },
    };

    let Rectangle {
        top_left: Point { x: top_left_x, y: top_left_y },
        bottom_right: Point { x: bottom_right_x, y: bottom_right_y },
    } = rect1;

    rect_area(rect1);

    let top_left: Point = Point {
        x: 5.0,
        y: 1.0,
    };

    let side: f32 = 5.0;

    let rect = square(top_left, side);

    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;
    let area = width * height;

    println!("Area of rectangle: {:?}", area);
}

fn rect_area(rect: Rectangle) {
    let Rectangle {
        top_left: Point { x: top_left_x, y: top_left_y },
        bottom_right: Point { x: bottom_right_x, y: bottom_right_y },
    } = rect;

    let width = bottom_right_x - top_left_x;
    let height = top_left_y - bottom_right_y;

    let area = width * height;

    println!("Area of rectangle: {}", area);
}

fn square(top_left: Point, side: f32) -> Rectangle {
    let rect = Rectangle {
        top_left: Point { x: top_left.x, y: top_left.y},
        bottom_right: Point { x: top_left.x + side, y: top_left.y - side}
    };

    return rect
}