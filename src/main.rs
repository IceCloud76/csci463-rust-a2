//******************************************************************
//
// CSCI 463 Assignment 2 Rust
//
// Author: IceCloud76
//
// Bitwise Operators
//
//******************************************************************
// use std::io;
use std::io::{self, Write};

fn main() {

//     let mut x: u32;
    let mut _x_str = String::new();
    
    //read in a value from the user from stdin
    print!("Input: ");
    let _ = io::stdout().flush(); //this is needed after a print! macro to allow stdin on the same line
    io::stdin()
        .read_line(&mut _x_str)
        .expect("Failed to read line");
    
    //convert the string to a u32 (uint32_t)
    let mut _x: u32 = match _x_str.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Failed to convert string to u32"),
        };

    print!("{}", _x_str);
    _x_str.clear();
    //infinite loop while the input is valid
    loop {
        
       // println!("0x {:04}", {10});

        print!("Input: ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut _x_str)
            .expect("Failed to read line");
        print!("{}", _x_str);
        _x_str.clear();
        //convert the string to a u32 (uint32_t)
        /*
        _x = match _x_str.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Failed to convert string to u32"),
            };
        */
    }

    /*
    while(std::cin.good()){ //while the hex value is valid
        
        std::cout << "0x" << std::setfill('0') << std::hex << std::setw(8) << x << " = ";
        printBinFloat(x);
        std::cin >> std::hex >> x;

    }
    
    */

}
