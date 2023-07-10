#![allow(non_snake_case)]
#![allow(unused_parens)]
use std::io;



//****************** STACK STRUCT **************************
//stack implementation I found by googling a bit
    struct Stack<T> {
        stack: Vec<T>,
    }
  
        impl<T> Stack<T> {

            fn new() -> Self {
            Stack { stack: Vec::new() }
            }
            
            fn pop(&mut self) -> Option<T> {
                self.stack.pop()
            }
            
            fn push(&mut self, item: T) {
                self.stack.push(item)
            }
            
            fn is_empty(&self) -> bool {
                self.stack.is_empty()
            }
            
            fn length(&self) -> usize {
                self.stack.len()
            }
        
            fn peek(&self) -> Option<&T> {
                self.stack.last()
            }

            fn peekf(&self) -> Option<&T> { //im not quite sure if the implementation of this functions
                                            //defeats the whole purpose of using a stack BUT i digress 
                return self.stack.first();
            }
        }

// **************** MAIN FUNCTION ***********************

fn main(){

    println!("Welcome to:");
    println!("**** THE EXTRAORDINARY RUST CALCULATOR *******\nStart by typing your data stuff");
    println!("If you need help type help \nIf you want to stop type stop");

    let mut input = String::new();
    let mut postfixBuffer= String::new();
    let mut data="0";
    let mut stack :Stack<char> = Stack::new();
    let mut result : u32;
    let listOfCalc= "+*-/^";

    // stack.push('1');
    // stack.push('1');   
    // get 2 numbers from stack and convert the chars to numbers
    // let test2=stack.pop().unwrap().to_digit(10).unwrap()+stack.pop().unwrap().to_digit(10).unwrap();
    // println!("EDW !!!!!!!!!!!!!!!! {:?}",test2);
    
    // REMINDER TO CLEAR THE BUFFER
    // add a mutable var ans?
    while("stop"!=data){// calculator loop

        input.clear();// clears the previous input
        println!("Awaiting input:");
        io::stdin().read_line(&mut input).unwrap(); //reading from keyboard
        data=input.trim();

        if ("help" == data) {
            menu();
        }
        else{
            //Detecting important chars for loop 
            //Convertion to postfix
            for c in data.chars(){
                postfixBuffer=postfixConvert(postfixBuffer,c,&mut stack,listOfCalc);
            }
            if(!stack.is_empty()){ // leftover operator
                postfixBuffer.push(stack.pop().unwrap());
            }
            println!("FINAL BUFFER {}",postfixBuffer);

            result=postfixCalc(&postfixBuffer,&mut stack,listOfCalc);
            // result=postfixCalc("12-3&45*6+7",&mut stack,listOfCalc);

            println!("\n****** FINAL RESULTS ********\nINFIX:{0} \nPOSTFIX:{2} \nRESULT:{1}",data,12,postfixBuffer);
//edw

        }
    }

}

// *********************** FUNCTIONS *************************************************************************************************
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

// ****************************************************************************************************************************

fn listCalc() ->i32{
    println!("Available calulations: \n +,- Addition Subtraction\n *,/ Multilpycation Division\n ^x \"To the power of x\"");
    println!(" sqrt(x) Square root of x\n sin(x),cos(x),tan(x) (x in degrees)\n logX(Y) log of Y with base X");
    //postfixBuffer must be done first
    return 2;
}

// ***************************************************************************************************************************

fn postfixConvert( mut dataString :String,c :char,stack :&mut Stack<char>,listOfCalc :&str)-> String{
    
    if (listOfCalc.find(c).is_some()){ 
        if (stack.is_empty()){ //prevents code panicking from the else if statement below
            stack.push(c);            
        }
        else if ( priority(c) <= priority( *stack.peekf().unwrap() ) ){
            while(!stack.is_empty()){
                dataString.push(stack.pop().unwrap());
            }
            stack.push(c);
        }
        else{ 
            stack.push(c);
        }
    }
    
    if ( c=='('){
        stack.push(c);
    }
    
    if (c.is_digit(10) ){        
        //push chars into string with push() ,the method push_str() requires a slice as input                                    
        dataString.push(c);    
    }

    if(c==')'){

        let mut temp=stack.pop().unwrap();
        dataString.push(temp);
        
        temp= stack.pop().unwrap();
        while(temp!='('){
            dataString.push(temp);
            temp=stack.pop().unwrap();
        }
    }
    
    // println!("postfix/dataString status ->{}",dataString);
    // println!("peek {:?}",stack.peek());
    return dataString;
}

// ****************************************************************************************************************************

fn postfixCalc(postData :&str,stack: &mut Stack<char>,listOfCalc :&str) -> u32{
    //stack begins empty
    // while(stack.is_empty()){
//     println!("statck :{:?}",stack.pop());
// }
    let mut result= 0;
    let mut charVar;

    for c in postData.chars(){

        println!("loop ...");
        if (c.is_digit(10) ){        
            stack.push(c);
            println!("number pushed {}",c);
            
        }
        else if(listOfCalc.find(c).is_some()){
            if (c=='+'){
                result=stack.pop().unwrap().to_digit(10).unwrap() + stack.pop().unwrap().to_digit(10).unwrap();
                
                println!("elo result {}",result);
                charVar= char::from_digit(result,10);
                println!("elo charvar {}",charVar.unwrap());
                stack.push(charVar.unwrap());
            }
            if (c=='-'){
                result= result + stack.pop().unwrap().to_digit(10).unwrap()-stack.pop().unwrap().to_digit(10).unwrap();
                
                println!("elo result {}",result);
                charVar= char::from_digit(result,10);
                println!("elo charvar {}",charVar.unwrap());
                stack.push(charVar.unwrap());
            }
            if (c=='*'){
                result= result + stack.pop().unwrap().to_digit(10).unwrap()*stack.pop().unwrap().to_digit(10).unwrap();
                
                println!("elo result {}",result);
                charVar= char::from_digit(result,10);
                println!("elo charvar {}",charVar.unwrap());
                
                stack.push(charVar.unwrap());
            }
            if (c=='/'){
                result= result + stack.pop().unwrap().to_digit(10).unwrap()/stack.pop().unwrap().to_digit(10).unwrap();
                
                println!("elo result {}",result);
                charVar= char::from_digit(result,10);
                println!("elo charvar {}",charVar.unwrap());
                stack.push(charVar.unwrap());
            }
        }
    }
    
    
    println!("CALCULATIONS FOR {0}\nRESULT IS:{1}",postData,result);
    return result;
}

// *************************************************************************************************************************

fn priority( operator :char) -> u32{
    match operator{
     '+'|'-'=>return 1,
     '*'|'/'=>return 2,
     '^'=>return 3,
    _=>return 0,
    }
}