#![allow(dead_code)]

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: i64,
}

fn main() {
    let book1: Book = Book {
        title: String::from("First Book"),
        author: String::from("Me"),
        year: 2025,
    };

    let book2: Book = Book {
        title: String::from("Some Book"),
        author: String::from("Not me"),
        year: 2020,
    };

    println!("{}", handle_book(book1));
    println!("{}", handle_book(book2));
}

fn handle_book(book: Book) -> String {
    let title = book.title;
    let author = book.author;
    let year = book.year;

    return format!("The {} book was written by {} in {}", title, author, year);
}