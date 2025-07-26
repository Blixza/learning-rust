#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Without destructuring
    // let name = person.name;
    // let age = person.age;
    
    // With destructuring
    let Person { name, age } = person;
    println!("Name: {}, Age: {}", name, age);
}