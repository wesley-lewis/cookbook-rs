#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Implementing panic
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

fn main() {
    compare_stmt_match(Some("Another Book")); 
    compare_stmt_match(Some("Rust Cookbook")); 

    compare_stmt_unwrap(Some("Rust Cookbook"));
    compare_stmt_unwrap(Some("Another Book"));
}

fn compare_stmt_match(input: Option<&str>) {
    match input {
        Some("Rust Cookbook") => println!("Rust cookbook was selected"),
        Some(inner) => println!("Rust cookbook was not selected"),
        None => println!("No input provided"),
    }
}

fn compare_stmt_unwrap(input: Option<&str>) {
    let inside_val = input.unwrap();
    if inside_val == "Another Book" {
        panic!("Rust Cookbook is not selected");
    }
    println!("I love {}s!!", inside_val);
}

fn panic_example() {
    compare_stmt("Rust Cookbook");
    compare_stmt("Another book");
}

fn compare_stmt(stmt: &str) {
    if stmt == "Another book" {
        panic!("Rust cookbook is not selected");
    }
    println!("Statements is {}!!", stmt);
}
