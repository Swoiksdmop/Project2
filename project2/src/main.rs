use std::io;
 fn main() {
    let mut x = String::new();
    println!("Is x 300000");
    io::stdin().read_line(&mut x).expect("failed to read line");
    println!("You typed: {x}");
    let x = x.trim();
    if x == "yes" {
        println!("Correct")
    } else {
        println!("Wrong")
    }
    // trim newline from x let x = x.trim(); if x == “yes” { println!(“Correct”); } else { println!(“Wrong”); } }
 }