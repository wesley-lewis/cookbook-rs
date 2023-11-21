#![allow(dead_code)]
#![allow(unused_variables)]

//-- ##################################
//-- Task: Waiting for a child process
//-- Author: Wesley Lewis
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #################################
//

use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();  
    let _result = child.wait().unwrap();

    print!("status if child process {} \n", _result);
    println!("reached end of main");
}

fn creating_child_process() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
        });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        println!("rustc succeeded and stdout was:n{}", s);
    }else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rust falied and stderr was:n{}", s);
    }
}

fn safe_mut_access() {
    let data = Arc::new(Mutex::new(vec![1,2,3,4]));
    for i in 0..10 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
            println!("Thread id: {:?}", i);
            println!("Data value: {:?}", data[0]);
        });

    }

    thread::sleep(Duration::from_millis(10));
}

fn channels() {
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

// pg 127
