use std::io::{self, stdin};

fn main(){
    let add = |x: i32,y: i32| x+y;
    let sub = |x: i32,y: i32| x-y;
    let mul = |x: i32,y: i32| x*y;
    let div = |x: i32,y: i32| x/y;


    println!("Welcome to Calculator");
    io::stdin().read_line(&mut a).expect("Failed to read");
    let a: i32 = a.trim().parse().expect("Enter valid number");

    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read");
    let b: i32 = b.trim().parse().expect("Enter a valid number");

    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read");
    let op: u32 = op.trim().parse().expect("Enter a valid number");

    match op {
        0 => println!("Result: {}",add(a,b)),
        1 => println!("Result: {}",sub(a,b)),
        2 => println!("Result: {}",mul(a,b)),
        3 => println!("Result: {}",div(a,b)),
        _ => println!("Invalid Choice"),
    };



}