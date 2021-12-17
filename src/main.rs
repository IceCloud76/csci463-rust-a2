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


/**
 * Main function
 *
 * Takes an input of a hexadecimal number and stores it in x. From there, it enters a loop in which we check if what the user typed
 * is valid, and we print it back to the screen. We then call printBinFloat(), to process the rest of the printing. Finally, we have the user enter
 * another valid x value
 * 
 * @return int of the return value
 *
 ********************************************************************************/
fn main() {

    let mut _x_str = String::new();
    
    //read in a value from the user from stdin
    print!("Input: ");
    let _ = io::stdout().flush(); //this is needed after a print! macro to allow stdin on the same line
    
    io::stdin()
        .read_line(&mut _x_str)
        .expect("Failed to read line");
    
//     let raw = "0x1f"; //eual to 31 in base 10
    //convert the string to a u32 (uint32_t)
    //cuts off 0x and the newline at the end
    let without_prefix = _x_str.trim_start_matches("0x").trim_end(); 
    let x = u32::from_str_radix(without_prefix, 16);
    println!("{:?}", x); //print in base 10
    
//     println!("{:#02x}", x.unwrap()); //x is a result object which uncontains a u32 object
    
    _x_str.clear(); //read_line appends to the string. Thus we must clear it out
    
    
    //infinite loop while the input is valid
    loop {
        
        print!("Input: ");
        let _ = io::stdout().flush();  //this is needed after a print! macro to allow stdin on the same line
        
        io::stdin()
            .read_line(&mut _x_str)
            .expect("Failed to read line");
       
       //convert the string to a u32 (uint32_t)
        
        let without_prefix = _x_str.trim_start_matches("0x").trim_end(); 
        let x = u32::from_str_radix(without_prefix, 16);
        println!("{:?}", x);
//         println!("{:#02x}", x.unwrap()); //x is a result object which uncontains a u32 object
        
        _x_str.clear(); //read_line appends to the string. Thus we must clear it out

        print_bin_float(x.unwrap());
    }

    
    /*
    while(std::cin.good()){ //while the hex value is valid
        
        std::cout << "0x" << std::setfill('0') << std::hex << std::setw(8) << x << " = ";
        printBinFloat(x);
        std::cin >> std::hex >> x;

    }
    
    */

}

/**
 * Prints a formated output
 *
 * Prints the hexadecimal value in binary on the first line, the sign bit on the second, 
 * the exponent in hex and decimal on the third, the sign in hex on the fourth, and finally
 * the full value of the number is printed last in a binary point value
 *
 * @param x a 32-bit hexadecimal value
 *
 *
 * @note This is how you can add an optional note about the function that
 *    may be of interest to someone using it.
 *
 * @warning Values of exponents ranging from 0 to 23 (inclusive) haven't been well tested. It may not work
 * Used in testing: 40c00002
 *
 * @bug This is how you can add an optional description of a known bug in the
 *    function such as: This only works for positive values of z.
 ********************************************************************************/
fn print_bin_float( _x : u32) {
    // C/C++ Bitwise Operators minute 29:40
    // https://www.youtube.com/watch?v=dx7ZQJZogMw
    //TODO 
}
