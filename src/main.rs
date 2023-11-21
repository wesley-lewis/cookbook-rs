#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Building my first macro
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

macro_rules! Check_Val {
    (x => $e:expr) => (println!("mode x: {}", $e));
    (y => $e:expr) => (println!("mode y: {}", $e));
}

fn main() {
    Check_Val!(y => "wesley");
}

