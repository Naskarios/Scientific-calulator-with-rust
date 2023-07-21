#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_variables)]
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
    let listOfOps= "+*-/^ ";
    let mut ans:f32=0.0;
        

    // data=clear clears ans
    println!("Welcome to:");
    println!("**** THE EXTRAORDINARY RUST CALCULATOR *******\nStart by typing your math stuff (ex. 1+2*3+4)");
    println!("or type some of the available commands:");
    println!("->menu(help)\n->stop\n->clear");
    println!("Note:This version of the calculator cannot respond accordingly to the use of parantheses \nplease avoid using them");

    while("stop"!=data){// calculator loop

        input.clear();// clears the previous input
        println!("Awaiting input:");
        io::stdin().read_line(&mut input).unwrap(); //reading from keyboard
        data=input.trim();

        if ("menu" == data) {
            ans=ans + menu();
        }
        else if ("clear" == data){
            ans=0.0;
        }
        else{
            //Detecting important chars for loop 
            //Convertion to postfix
            let mut tempC=' ';
            for c in data.chars(){
                postfixBuffer=postfixConvert(postfixBuffer,tempC,c,&mut stack,listOfOps);
                tempC=c;
            }

            if(postfixBuffer.chars().nth_back(0).unwrap().is_digit(10)){
                postfixBuffer.push('~');
            }

            while(!stack.is_empty()){ // leftover operator if it exists
                postfixBuffer.push(stack.pop().unwrap());
            }
            println!("FINAL BUFFER {}",postfixBuffer);

            result=postfixCalc(&postfixBuffer,&mut sStack,listOfOps);
            println!("CALCULATIONS FOR {0}\nRESULT IS:{1}",postfixBuffer,result);

            postfixBuffer.clear();
            ans=ans + result;
        }
        println!("ANS={}",ans);
    }
}

//***************************************************************************************************************************
fn postfixConvert( mut dataString :String,tempC :char,c :char,stack :&mut Stack<char>,listOfOps :&str)-> String{
    // debug
    println!{"function for char ->{} \ntemp= {}",c,tempC};
    //if(temp.matchOps and c.matchOps)
    // throw error

    if (c.is_digit(10) || c== '.' ){        
        //push chars into string with push() ,the method push_str() requires a slice as input                                    
        dataString.push(c);    
    }
    if ( c=='('){
    stack.push(c);
    //debug
    println!("pushed {}",c);
    }
    
    if (c==')'){

        if ( tempC.is_digit(10) ){ 
            dataString.push('~');
        }
        // dataString.push(opsInStack);
        let mut opsInStack=stack.pop().unwrap();
        //debug
        println!("poped");
        
        while(opsInStack!='(' && stack.peek().is_some()){
            
            dataString.push(opsInStack);
            opsInStack=stack.pop().unwrap();
            //debug
            println!("poped");
        }
    }

    else if (listOfOps.find(c).is_some()){
        
        if ( tempC.is_digit(10) ){
            dataString.push('~'); //if this statement above is true then on god we have completed number ex "1.2+"
                              //the operators and ')' split the numbers
        }


        if (stack.is_empty()){ //prevents code panicking from the else if statement below
                                //the .peekf.unwrap cause the panick
            stack.push(c);     
            //debug
            println!("pushed {}",c);
        }
        else if ( priority(c) <= priority( *stack.peekf().unwrap() ) ){
		// output string until empty or '(' found
        let mut poped =stack.pop().unwrap();
            while(!stack.is_empty() && poped !='('){
                dataString.push(poped);
                poped =stack.pop().unwrap();
                //debug
                println!("POPed / and pusghed {}",poped);
            }
            stack.push(c);
            //debug
            println!("pushed {}",c);
        }
        else{ 
            stack.push(c);
            //debug
            println!("pushed {}",c);
        }
    }
    

    //debug
    println!("postfix/dataString status ->{}",dataString);
    return dataString;
}


//***************************************************************************************************************************
fn postfixCalc(postData :&str,sStack: &mut Stack<String>,listOfOps :&str) -> f32{
    //the stack begins empty

    let mut result= 0.0;
    let mut tempString ="".to_string();

    for c in postData.chars(){
        
        if (c.is_digit(10) || c=='.'){        
            //debug
            println!("number pushed into temp {}",c);
            tempString.push(c);
        }
        else if (c=='~'){
            //debug
            println!("pushing string -> {}",tempString);
            sStack.push(tempString);
            tempString= String::from("");
        }
        else if (listOfOps.find(c).is_some()){

          result=doTheThing(c,sStack,result);
        }
    }
    return result;
}

//***************************************************************************************************************************
fn doTheThing(c :char,sStack :&mut Stack<String>,mut result : f32)->f32{
    
    
        if (c=='+'){

            let  s1 =sStack.pop().unwrap();//taking the strings
            let  s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   //converting them into numbers
            let num2: f32 = s2.parse().unwrap();   

            result= num1 + num2;
            
            sStack.push(result.to_string());       
        }
        if (c=='-'){
            let  s1 =sStack.pop().unwrap();
            let  s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num2 - num1;
            
            sStack.push(result.to_string());   
        }
        if (c=='*'){
            let  s1 =sStack.pop().unwrap();
            let  s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num1 * num2;
            
            sStack.push(result.to_string());  
        }
        if (c=='/'){
            let  s1 =sStack.pop().unwrap();
            let  s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num2 / num1;
            
            sStack.push(result.to_string()); 
        }
        if (c=='^'){
            let  s1 =sStack.pop().unwrap();
            let  s2 =sStack.pop().unwrap();

            let num1: f32 = s1.parse().unwrap();   
            let num2: f32 = s2.parse().unwrap();   

            result= num2.powf(num1);
            sStack.push(result.to_string());
        }
        //debug
        println!("Pushed result {1}->{0}",result.to_string(),c);
        return result;
    }
//***************************************************************************************************************************
fn menu() ->f32{

    let menuChoice;
    let mut result=0.0;
    let mut input = String::new();

    println!("1.List available calculations\n2.About the calc\n3.Help\n");
    println!("Select option:");
    
    // implementing input
    io::stdin().read_line(&mut input).unwrap();
    menuChoice=input.trim();

    //debug
    // println!("Input= {}",menuChoice); 
    
        match menuChoice{
            "1"|"list"=>result=listCalc(),
            "2"|"about"=>println!("This project was made for the sole reason of me getting my first taste of rust programming"),
            "3"|"help"=>println!("**DONT USE PARANTHESES**\nAnswer(ANS) is the result of the previous calculation it can be cleared with 'clear'\nUse of extra functions (ex. cos(x) ) will be added to ANS after selection "),
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
fn listCalc() ->f32{
    let mut input = String::new();

    println!("Additionall calculations: \n1.sqrt(x) Square root of x\n2. sin(x) (x in radians)\n3.cos(x) (x in radians)\n4.tan(x) (x in radians)\n5.logX(Y) log of Y with base X\nChoose:");
    io::stdin().read_line(&mut input).unwrap();
    let binding = input.clone(); //thank the compiler who told me to write the lines
    let choice=binding.trim();
    input.clear(); //for 1 as choice and 4 as x we have this string in the input value
                    // "1\r\n4\r\n" using trim() we remove some of the characters but not the characters in between 1 and 4
                    //so we clear the string
    println!("Give the value of x:");
    io::stdin().read_line(&mut input).unwrap();
    
    let x :f32 =input.clone().trim().parse().unwrap();

    match choice{
        "1"=>return x.sqrt(),
        "2"=>return x.sin(),
        "3"=>return x.cos(),
        "4"=>return x.tan(),
        "5"=>return x.log10(),
       _=>return 0.0,
    
    }

}
//***************************************************************************************************************************
// fn firstCheck()->i32{

//     //checks if the calculations are written correctly
//     //must also check if there are double dots (ex. 1..2-> 1.2)
//     // if(dataString.matches(listOfOps)){
//     // return 0;//ok
//     // }
//     // else{
//     // return 1;//skata
//     // }
//     return 12;
// }

