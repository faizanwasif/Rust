fn main() {
    // Functions

    my_function();
    my_func_with_args(10, 20);
    let a = my_func_with_return(5, 4);
    let b = my_func_with_return_2(1, 2);

    print!("The value of a is: {}\n", a);
    print!("The value of b is: {}\n", b);

    
    // Code Blocks
    code_block();


    // Conditional Statements
    if_else();
    let word = if_else_if_else("hello");
    print!("The value of word is: {}\n", word);

    let word = if_else_if_else("bye");
    print!("The value of word is: {}\n", word);

    match_statement();

    // Loops
    loop_func();
    while_loop();
    for_loop();


}



/*
    1) Functions
*/

fn my_function(){
    println!("This is my function");
}

fn my_func_with_args(x:i32, y:i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn my_func_with_return(x:i32, y:i32) -> i32{
    return 12; // return keyword is optional
    print!("This will not be printed");
    print!("You can also return without using return keyword but don't put semicolon at the end of the statement");
    x + y // This will be returned is you remove the above return statement
}

fn my_func_with_return_2(x:i32, y:i32) -> i32{
    10 
}


/*
    2) Code Blocks
*/


fn code_block(){
    let code_block = {
        let x = 1;
        let y = 2;
        x + y
    };
    print!("The value of code_block is: {}\n", code_block);
}


/* 
    3) Conditional Statements
        - if else
        - if else if else ladder
        - match
*/

fn if_else(){
    let mut x = 2;
    if x == 2{
        x = 6;
    }
    else{
        x = 7;
    }
    print!("The value of x is: {}\n", x);
}

fn if_else_if_else(x:&str)->&str{
    let word = if x == "hello"{
        print!("Hello there\n");
        "hi"
    }
    else if x == "bye"{
        print!("Goodbye\n");
        "bye"
    }
    else{
        print!("No match found\n");
        "not found"
    };
    return word;
}

fn match_statement(){
    let mut grade = 'N';
    let marks = 45;
    grade=match marks{
        0..=60 => 'F',
        61..=70 => 'E',
        71..=80 => 'D',
        81..=90 => 'C',
        91..=95 => 'B',
        96..=100 => 'A',
        _ => 'N'
    };
    print!("The grade is: {}\n", grade);
}


/*
    4) Loops
        - for loop
        - while loop
        - loop
*/

fn loop_func(){
    'outer :loop{
        print!("This is an infinite loop\n");
        break 'outer;
    }
}

fn while_loop(){
    let mut x = 0;
    while x<10{
        print!("The value of x is: {}\n", x);
        x+=1;
    }
    print!("While loop ended\n");
}

fn for_loop(){
    for x in 0..10{
        print!("The value of x is: {}\n", x);
    }
    print!("For loop ended\n");
}