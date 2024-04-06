use std::io;

fn main() {
    // prints a message
    println!("Guess the number!");
    println!("Please input your guess.");

    // we'll create a mutable variable
    // By default variable are not mutable
    let mut guess = String::new();


    /* RECEIVING USER INPUT
    If we hadn’t imported the io library with use std::io; at the beginning of the program,
     we could still use the function by writing this function call as std::io::stdin.
    The stdin function returns an instance of std::io::Stdin,
     which is a type that represents a handle to the standard input for your terminal.
    The line .read_line(&mut guess) calls the read_line method on the standard input handle
     to get input from the user. We’re also passing &mut guess as the argument to read_line
     to tell it what string to store the user input in. The full job of read_line is to take
     whatever the user types into standard input and append that into a string
     (without overwriting its contents), so we therefore pass that string as an argument.
    The string argument needs to be mutable so the method can change the string’s content.
    The & indicates that this argument is a reference, which gives you a way to let multiple
     parts of your code access one piece of data without needing to copy that data into memory
     multiple times. like variables, references are immutable by default.
    Hence, you need to write &mut guess rather than &guess to make it mutable.
    read_line returns a Result value. Result is an enumeration, often called an enum,
     which is a type that can be in one of multiple possible states. We call each possible
     state a variant.
    Result’s variants are Ok and Err. The Ok variant indicates the operation was successful,
     and inside Ok is the successfully generated value. The Err variant means the operation
     failed, and Err contains information about how or why the operation failed.
    An instance of Result has an expect method that you can call. If this instance of Result
     is an Err value, expect will cause the program to crash and display the message that you
     passed as an argument to expect. If the read_line method returns an Err, it would likely
     be the result of an error coming from the underlying operating system.
    If this instance of Result is an Ok value, expect will take the return value that Ok
     is holding and return just that value to you so you can use it. In this case,
     that value is the number of bytes in the user’s input.
     */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    /* PRINTING VALUES WITH println! PLACEHOLDERS
    The {} set of curly brackets is a placeholder: think of {} as little crab pincers
     that hold a value in place. When printing the value of a variable, the variable name
     can go inside the curly brackets. When printing the result of evaluating an expression,
     place empty curly brackets in the format string, then follow the format string with a
     comma-separated list of expressions to print in each empty curly bracket placeholder
     in the same order.
    ```
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
    ```
     */
    println!("You guessed: {guess}");
}