#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping
    use crate::Stage::{Beginner, Advanced};
    // Automatically `use` each name inside `Role`
    use crate::Role::*;

    // Equivalent to `Stage::Beginner`
    let stage = Beginner;
    // Equivalent to `Role::Student`
    let role = Student;

    match Stage {
        // Note the lack of scoping because of the explicit `use` above
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match Role {
        // Note again the lack of scoping
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}