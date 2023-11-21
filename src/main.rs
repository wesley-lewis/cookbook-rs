#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Implementing map 
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

#[derive(Debug)]
enum Food { Apple, Carrot, Potato }

#[derive(Debug)] 
struct Peeled(Food);

#[derive(Debug)] 
struct Chopped(Food);

#[derive(Debug)] 
struct Cooked(Food);


fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible.")
    }
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
