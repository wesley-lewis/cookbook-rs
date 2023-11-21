#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Building my first macro
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

macro_rules! Wesley_Lewis {
    () => {
        println!("Rust macros are cool");
    }    
}

fn main() {
    Wesley_Lewis!() ;
}

