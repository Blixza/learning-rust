use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug)]
struct MyString {
    value: String,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl From<Vec<i32>> for Rectangle {
    fn from(item: Vec<i32>) -> Self {
        Rectangle { width: item[0], height: item[1] }
    }
}

impl From<i32> for Point {
    fn from(item: i32) -> Self {
        Point { x: item, y: item }
    }
}

impl From<&str> for MyString {
    fn from(item: &str) -> Self {
        MyString { value: String::from(item) }
    }
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item}
    }
}

fn main() {
    let num = Number::from(5);
    let my_string = MyString::from("hello");
    let p: Point = Point { x: 5, y: 2 };
    let rect = Rectangle::from(vec![10, 5]);
    println!("My number is {:?}", num);
    println!("My string is {:?}", my_string);
    println!("my point is {:?}", p);
    println!("my rect is {:?}", rect);
}