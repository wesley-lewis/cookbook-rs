#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Implementing designators
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
        println!("You called {:?}()", stringify!($func_name))
    }
    )
}

macro_rules! print_result {
    ($expression:expr) => (
    println!("{:?} = {:?}", stringify!($expression),$expression)
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

create_function!(foo);
create_function!(bar);
