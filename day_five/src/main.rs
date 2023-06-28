use std::{
    fmt::Display,
    io::{self, BufRead},
    sync::{self, mpsc},
    thread,
};

fn main() {
    constait_and_trait_bounds();
    standard_input();
    threads();
    message_passing();
    // generic_types();
}

// struct Pair<T> {
//     first: T,
//     second: T,
// }
// fn generic_types() {
//     let pair_of_integres = Pair {
//         first: 1,
//         second: 2,
//     };

//     let pair_of_strings = Pair {
//         first: "Hello",
//         second: "World",
//     };
//     println!(pair_of_integres);
//     println!(pair_of_strings);
// }

// fn print_pair<T>(pair: Pair<T>) {
//     println!("First: {}, Second: {}", pair.first, pair.second); // error here
// }

fn print_and_return<T: Display>(value: T) -> T {
    println!("Value: {}", value);
    value
}

fn constait_and_trait_bounds() {
    let value = print_and_return(42);
    print!("Returned value: {}", value);
}

fn standard_input() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    println!("Enter your name:");

    stdin
        .lock()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    println!("Hello, {}!", buffer);
}

fn threads() {
    let thread = thread::spawn(|| {
        println!("Hello from thread 1");
    });

    let thread1 = thread::spawn(|| {
        println!("Hello from thread 2");
    });

    thread1.join().expect("Thread 1 panicked");
    thread.join().expect("Thread 2 panicked");
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    let thread1 = thread::spawn(move || {
        let message = String::from("Hello from Thread 1!");
        tx.send(message).expect("sending message failed");
    });

    let thread2 = thread::spawn(move || {
        let received = rx.recv().expect("Receiving message failed");
        println!("{}", received);
    });
    thread1.join().expect("Thread 1 panicked");
    thread2.join().expect("Thread 2 panicked");
}
