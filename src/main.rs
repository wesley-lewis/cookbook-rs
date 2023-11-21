#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Routing using nickel
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//
#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get("/user/:userid", middleware! {
        |request| format!("This is user: {:?}", request.param("userid"))
    });
    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/a/*/d", middleware!("This matches a/b/d and also a/b/c/d"));
    server.get("/a/**/d", middleware! ("this matches /a/b/d and also /a/b/c/d"));

    let _ = server.listen("127.0.0.1:3000");
}
