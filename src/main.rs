use rand::{self, Rng}; //https://crates.io/crates/rand

fn main() {
    println!("Hello, world! This is my first Rust program!");
    println!("What is your name?");
    let mut username = get_input();
    println!("Testing...");
    username = scramble(username);//note: the ownership of username is being passed into this function
    println!("\nWelcome, {}!\nOur first test will be collecting input and factorializing the number obtained. Then, we will move on to finding Fibonacci numbers!",username);
    //initializing variables
    let mut input_int: u8;
    let mut result: u128;
    //entering the "play again" loop/break for factorializing.
    loop {
        input_int = update_number();
        result = factorial(u128::from(input_int));
        println!("Printing {}!: {}",input_int,result);       
        if !update_play_again(){
            break
        }
    }
    //this time I am testing the while loop and the fibonacci number.
    println!("Moving on to the fibonacci sequence.\n");
    let mut play_again = true;
    while play_again {
        input_int = update_number();
        result = get_fibonacci(input_int);
        println!("Printing the {} fibonacci number is: {}",input_int,result);
        play_again = update_play_again(); 
    }
    print!("Goodbye, {}!",username);
}
//factorialization function. It is using an unsigned 128 bit numbers, which can correctly handle input from 1 to 34.
fn factorial(received_integer:u128) -> u128 {
    if received_integer >= 35{
        println!("I am sorry, but this function cannot handle numbers above 34. Returning 0 instead.");
        return 0
    }
    if received_integer <= 0 {
        1
    }
    else {
        received_integer * factorial(received_integer-1)
    }
}
//this is how I receive input for a calculation
fn update_number() -> u8 {
    println!("Please enter a number for the next calculation:\n");
    println!("Reminder, only integers from 0 to 255 are allowed, and numbers above 34 are not recommended.");
    let trimmed_string = get_input();
    trimmed_string.parse::<u8>().unwrap() //I learned how to convert string to int in Rust from stackoverflow: https://stackoverflow.com/questions/27043268/convert-a-string-to-int
}
//checks is the player wants to play again
fn update_play_again() -> bool {
    println!("Would you like to test another number? Enter yes or no.\n");
    let input_string = get_input();
    let input_character = &input_string[0..=0];//I am using splicing here to obtain just the first letter of the user input
    input_character == "y" || input_character == "Y"
}
//gets input, trims it, then returns the string
fn get_input() -> String {
    let mut input_string = String::new();//this is a mutable variable
    std::io::stdin().read_line(&mut input_string).unwrap();//I do not know what unwrap() mean or why it is used. I am just following along with information I found on tutorialspoint for now.
    let trimmed_string = input_string.trim(); //this is an immutable variable.
    trimmed_string.to_string()
}
//Caclulates the fibonacci number through recursion. Numbers over 40 take a long time.
fn get_fibonacci(count:u8) -> u128 {
    if count < 1 {
        return 0
    }
    if count == 1 {
        1
    }
    else {
        get_fibonacci(count - 2) + get_fibonacci(count-1)
    }
}
//takes a word and returns a slightly scrambled version. 
fn rearrange(prescrambled_word:String) -> String{
    let final_index = prescrambled_word.len() -1;
    let random_int = rand::thread_rng().gen_range(1..final_index); //https://stackoverflow.com/questions/19671845/how-can-i-generate-a-random-number-within-a-range-in-rust
    let firsthalf = &prescrambled_word[0..random_int];//demonstration of splicing
    let shuffled_letter = &prescrambled_word[random_int..=random_int];
    let secondhalf=&prescrambled_word[random_int+1..=final_index];
    let mut new_word = secondhalf.to_string();
    new_word.push_str(shuffled_letter);
    new_word.push_str(firsthalf);
    new_word.to_string()
}
//takes a word and sends it through the rearrange function. The longer the word, the more thoroughly it mixes the word.
fn scramble(prescrambled_word:String) -> String{
    let mut scrambled_word = prescrambled_word;//the ownership of prescrambled_word has been moved to scrambled_word and prescrambled_word is now empty  
    let count = scrambled_word.len();
    for _index in 0..count{//demonstration of a nested for loop
        scrambled_word = rearrange(scrambled_word);
        for char in scrambled_word.chars() {
            print!("{} ",char);
        }
        println!();
    }
    scrambled_word
}