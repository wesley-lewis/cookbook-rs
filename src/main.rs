#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: JSON handling in nickel
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//
#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};


fn main() {
    let mut server = Nickel::new();
    

    server.get("/", middleware! { "hello world" });
    

    let _ = server.listen("127.0.0.1:3000");
}
