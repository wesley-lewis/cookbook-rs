#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Implementing try! 
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = r#try!(vec.first()
        .ok_or("please use a vec with atleast one element".to_owned()));

    let value = r#try!(first.parse::<i32>()
        .map_err(|e| e.to_string()));

    Ok(2 * value)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first double is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93"];

    print(double_first(empty));
    print(double_first(strings));
}
//
// fn multiple_errors() {
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];
//     print(double_first(empty));
//     print(double_first(strings));
// }
// type Result<T> = std::result::Result<T, String>;
//
// fn double_first(vec: Vec<&str>) -> Result<i32> {
//     vec.first()
//         .ok_or("Please use a vector with atleast one element".to_owned())
//         .and_then(|s| s.parse::<i32>()
//             .map_err(|e| e.to_string())
//             .map(|i| 2 * i))
// }
//
// fn print(result: Result<i32>) {
//     match result {
//         Ok(n) => println!("The first double is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// type AliasedResult<T> = Result<T, ParseIntError>;
//
// fn aliases() {
//     print(double_number("10"));
//     print(double_number("tt"));
// }
//
// fn double_number(number_str: &str) -> AliasedResult<i32> {
//     number_str.parse::<i32>().map(|n| 2 * n)
// }
//
// fn print(result: AliasedResult<i32>) {
//     match result {
//         Ok(n) => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }


// fn impl_map_for_result() {
//     let twenty = double_number("10");
//     print(twenty);
//
//     let tt = double_number_map("t");
//     print(tt);
// }
// fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
//     match number_str.parse::<i32>() {
//         Ok(n) => Ok(2 * n),
//         Err(e) => Err(e),
//     }
// }
//
// fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
//     number_str.parse::<i32>().map(|n: i32| 2 * n)
// }
//
// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n) =>println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

fn impl_and_then() {
    let (cordon_blue, steak, sushi) = (Food::CordonBlue, Food::Steak, Food::Sushi);

    eat(cordon_blue, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
#[derive(Debug)]
enum Food { CordonBlue, Steak, Sushi }

#[derive(Debug)]
enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBlue => None,
        _ => Some(food),
    }
}

fn cookable(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable(food) {
        Some(food) => println!("Yay! on {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

// fn implimented_map() {
//     let apple = Some(Food::Apple);
//     let carrot = Some(Food::Carrot);
//     let potato = None;
//
//     let cooked_apple = cook(chop(peel(apple)));
//     let cooked_carrot = cook(chop(peel(carrot)));
//     let cooked_potato = process(potato);
//
//     eat(cooked_apple);
//     eat(cooked_carrot);
//     eat(cooked_potato);
// }

// #[derive(Debug)]
// enum Food { Apple, Carrot, Potato }

#[derive(Debug)] 
struct Peeled(Food);

#[derive(Debug)] 
struct Chopped(Food);

#[derive(Debug)] 
struct Cooked(Food);
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

// fn eat(food: Option<Cooked>) {
//     match food {
//         Some(food) => println!("Mmm. I love {:?}", food),
//         None => println!("Oh no! It wasn't edible.")
//     }
// }

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
