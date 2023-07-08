#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::io;

// **************** MAIN FUNCTION ***********************

fn main(){

    println!("Welcome to:");
    println!("**** THE EXTRAORDINARY RUST CALCULATOR *******\nStart by typing your data stuff");
    println!("If you need help type help \nIf you want to stop type stop");

    let mut input = String::new();
    let mut data="0";
    let mut buffer= String::new();
    while("stop"!=data){
        input.clear();// clears the previous input
        println!("Awaiting input:");
        io::stdin().read_line(&mut input).unwrap();
        data=input.trim();

        if ("help" == data) {

            menu();

        }
        else{
            
            //Detecting important chars for loop 
            for c in data.chars(){
                    if (c.is_digit(10) || c=='+' || c=='-' || c=='*' || c=='/' || c=='^' || c=='(' || c==')' ){
                        buffer.push(c);    //push chars into string ,the method push_str() requires a slice as input
                                            // push is only for chars
                        println!("buffer status ->{}",buffer)
                    }
                }
            //a lot of stuff to be done later (buffer?)
            // buffers evolution is gonna be the infix/postfix stack Credits  to Prof.Giannopoulos at Uni of Peloponnhsoy for teaching me that
        }
    }
    
}

// *********************** FUNCTIONS ***************************

fn menu() ->i32{

    let menuChoice;
    let mut result=0;
    let mut input = String::new();

    println!("1.List available calculations\n2.About the calc\n3.Help\n");
    println!("Select option:");
    
    // implementing input
    io::stdin().read_line(&mut input).unwrap();
    menuChoice=input.trim();

    // println!("Input= {}",menuChoice); //debug
    
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
    println!("Available calulations: \n +,- Addition Subtraction\n *,/ Multilpycation Division\n ^x \"To the power of x\"");
    println!(" sqrt(x) Square root of x\n sin(x),cos(x),tan(x) (x in degrees)\n logX(Y) log of Y with base X");
    //buffer must be done first
    return 2;
}