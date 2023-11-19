#![allow(dead_code)]
//-- ##################################
//-- Task: Passing values to a thread in Rust 
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut join_handles: Vec<JoinHandle<i32>> = Vec::new();

    for i in 0..10 {
        let join_handle = thread::spawn(move || {
            println!("Iteration: {}", i);
            i
        });

        join_handles.push(join_handle);
    }

    let jhs = thread::spawn(move || {
        for jh in join_handles {
            match jh.join() {
                Ok(val) => println!("Thread {} finished", val),
                Err(_) => println!("Error encountered"),
            }
        }
    });

    let _ = jhs.join();
}

fn code1() {
    let x = 1; 
    let handle = thread::spawn(move || {let y = x;});
    println!("{:?}", handle.join().unwrap());

    thread::spawn(|| {
        println!("Hello world from first thread, technically second thread");
    });

    let join_handle = thread::spawn(|| {
        println!("Hello world from third thread");
        3
    });

    match join_handle.join() {
        Ok(x) => println!("Third thread returned: {}", x),
        Err(_) => println!("Third thread panicked"),
    }
}

