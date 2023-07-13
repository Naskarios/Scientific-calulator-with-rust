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
//***************************************************************************************************************************

fn main(){
    let mut input = String::new();
    let mut postfixBuffer= String::new();
    let mut data="0";
    let mut stack :Stack<char> = Stack::new();
    let mut sStack :Stack<String> = Stack::new();
    let mut result : f32;   //changed from int to f32 for float math
    let listOfOps= "+*-/^";
    let listOfCalc= "sin cos log";// sin or sin( ?

        // REMINDER TO CLEAR THE BUFFER after every loop
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
            let mut tempC=' ';
            for c in data.chars(){
                postfixBuffer=postfixConvert(postfixBuffer,tempC,c,&mut stack,listOfOps);
                tempC=c;
            }
            postfixBuffer.push('~');
            if(!stack.is_empty()){ // leftover operator if it exists
                postfixBuffer.push(stack.pop().unwrap());
            }
            println!("FINAL BUFFER {}",postfixBuffer);

            
            result=postfixCalc(&postfixBuffer,&mut sStack,listOfOps);

            postfixBuffer.clear();
        }
    }
}

//***************************************************************************************************************************

fn postfixConvert( mut dataString :String,tempC :char,c :char,stack :&mut Stack<char>,listOfOps :&str)-> String{
    
    //if(temp.matchOps and c.matchOps)
    // throw error

    if (c.is_digit(10) || c== '.' ){        
        //push chars into string with push() ,the method push_str() requires a slice as input                                    
        dataString.push(c);    
    }
    else if (listOfOps.find(c).is_some()){

        dataString.push('~'); //if this statement above is true then on god we have completed number ex "1.2+"
                              //the operators and ')' split the numbers

        if (stack.is_empty()){ //prevents code panicking from the else if statement below
                                //the .peekf.unwrap cause the panick
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

    if (c==')'){

        // if ( tempC != ')'){ //should be on but causes this 1~2~+~3~4~5~/*-~6
            //                                                               ^  an ~ after opernad
            // if this turns off the the 2 will not have ~
        //     dataString.push('~');
        // }
        let mut opsInStack=stack.pop().unwrap();
        dataString.push(opsInStack);
        
        opsInStack= stack.pop().unwrap(); //this line prevents from duplicating operands in the buffer
                                    //comment the line and test  ((1 + 2) – 3 * (4 / 5)) + 6
        while(opsInStack!='('){
            dataString.push(opsInStack);
            opsInStack=stack.pop().unwrap();
        }
        
    }
    // println!("\t mphke {:?}",stack.peek());
    // println!("postfix/dataString status ->{}",dataString);
    return dataString;
}


//***************************************************************************************************************************


fn postfixCalc(postData :&str,sStack: &mut Stack<String>,listOfCalc :&str) -> f32{
    //the stack begins empty

    let mut result= 0.0;
    let mut tempString ="".to_string();


    for c in postData.chars(){
        
        println!("loop ...");
        if (c.is_digit(10) || c=='.'){        
            println!("number pushed into temp {}",c);
            tempString.push(c);
        }
        else if (c=='~'){
            println!("pushing string -> {}",tempString);
            sStack.push(tempString);
            tempString= String::from("");
        }
        else if (listOfCalc.find(c).is_some()){
          result=doTheThing(c,listOfCalc,sStack,result);
        }
    }
    
    
    println!("CALCULATIONS FOR {0}\nRESULT IS:{1}",postData,result);
    return result;
}


//***************************************************************************************************************************

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



//***************************************************************************************************************************
fn priority( operator :char) -> u32{
    match operator{
     '+'|'-'=>return 1,
     '*'|'/'=>return 2,
     '^'=>return 3,
    _=>return 0,
    }
}
//***************************************************************************************************************************

fn listCalc() ->i32{
    println!("Available calulations: \n +,- Addition Subtraction\n *,/ Multilpycation Division\n ^x \"To the power of x\"");
    println!(" sqrt(x) Square root of x\n sin(x),cos(x),tan(x) (x in degrees)\n logX(Y) log of Y with base X");
    //postfixBuffer must be done first
    return 2;
}
//***************************************************************************************************************************

fn firstCheck()->i32{

    //checks if the calculations are written correctly
    //must also check if there are double dots (ex. 1..2-> 1.2)
    // if(dataString.matches(listOfOps)){
    // return 0;//ok
    // }
    // else{
    // return 1;//skata
    // }
    return 12;
}

//***************************************************************************************************************************

fn doTheThing(c :char,listOfCalc:&str,sStack :&mut Stack<String>,mut result : f32)->f32{
    

    // else if (listOfCalc.find(c).is_some()){
        if (c=='+'){

            let mut s1 =sStack.pop().unwrap();//taking the strings
            let mut s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   //converting them into numbers
            let num2: f32 = s2.parse().unwrap();   

            result= num1 + num2;
            
            sStack.push(result.to_string());
            
            
        }
        if (c=='-'){
            let mut s1 =sStack.pop().unwrap();
            let mut s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num2 - num1;
            
            sStack.push(result.to_string());
            
        }
        if (c=='*'){
            let mut s1 =sStack.pop().unwrap();
            let mut s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num1 * num2;
            
            sStack.push(result.to_string());
            
        }
        if (c=='/'){
            let mut s1 =sStack.pop().unwrap();
            let mut s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num1 / num2;
            
            sStack.push(result.to_string());
            
        }
        if (c=='^'){
            let mut s1 =sStack.pop().unwrap();
            let mut s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num2.powf(num1);
            
            sStack.push(result.to_string());
            
        }
        println!("Pushed result ->{}",result.to_string());
        return result;
    }
