#![allow(dead_code)]
//-- ##################################
//-- Task: Channels in Rust 
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

fn main() {
    let threads = 10;
    let mut thread_holder = vec![];
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for thread_no in 0..threads {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(thread_no).unwrap();
            println!("thread {} finished", thread_no);
        });
    }
    
    for _i in 0..threads {
        thread_holder.push(rx.recv());
    }
    

    println!("{:?}", thread_holder);
}

fn spawning_joining_threads() {
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

