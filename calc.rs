#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::io;

// **************** MAIN FUNCTION ***********************

fn main(){

    println!("Welcome to:");
    println!("**** THE EXTRAORDINARY RUST CALCULATOR *******\nStart by typing your math stuff");
    println!("If you need help type help");

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    
    if ("help" == input_string.trim()) {
        menu();
    }
    else{
        //a lot of stuff to be done later (buffer?)
    }
    

}

// *********************** FUNCTIONS ***************************

fn menu() ->i32{

    let menuChoice;
    let mut result=0;
    let mut input_string = String::new();

    
    println!("1.List available calculations\n2.About the calc\n3.Help\n");
    println!("Awaiting input...");
    
    // implementing input
    io::stdin().read_line(&mut input_string).unwrap();
    menuChoice=input_string.trim();

    println!("Input= {}",menuChoice); //debug
    
    
    match menuChoice{
        "1"|"list"=>result=listCalc(),
        "2"|"about"=>println!("I did it"),
        "3"|"help"=>println!("im not helping"),
        _=>println!("Select only  the options shown above"),
        }
        return result;
    
}
// **********************************************************
fn listCalc() ->i32{
    //buffer must be done first
    return 2;
}