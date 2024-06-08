fn main() {
    // if Expressions

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    let number = 6;
    // concatenate if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // using in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    /*********************************************************************
     REPETITION WITH LOOPS
     *********************************************************************/
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        println!("loop: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // loop into an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // more secure loop for array
    // FOR LOOP
    for element in a {
        println!("The value is: {element}");
    }

    // range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
