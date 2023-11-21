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
    compare_stmt("Rust Cookbook");
    compare_stmt("Another book");
}

fn compare_stmt(stmt: &str) {
    if stmt == "Another book" {
        panic!("Rust cookbook is not selected");
    }
    println!("Statements is {}!!", stmt);
}
