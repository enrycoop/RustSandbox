fn main() {

    /*********************************************************************/
    /* SCALAR TYPES */
    /*********************************************************************/

    // INTEGER
    // can be signed (i8, i16, i32, i64, i128, isize)
    // unsigned (u8, u16, u32, u64, u128, usize)
    // usize and isize depends by architecture and are used to index
    // some sort of collection.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The number parsed is {guess}");
    // either you can unse _ for a visual separator

    let n: u32 = 1_000;
    println!("The annotated value is {n}");

    // FLOATING POINT
    let x = 2.0; // double precision float

    let y: f32 = 3.0; // single precision float

    // NUMERIC OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;


    // THE BOOLEAN TYPE
    let t = true;

    let f: bool = false; // with explicit type annotation

    // CHARACTER TYPE
    // char literals with single quotes is four bytes in size and repr. a
    // Unicode scalar value, which means it can represent a lot more than just ASCII.
    let c = 'z';

    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    /*********************************************************************/
    /* COMPOUND TYPES */
    /*********************************************************************/
    // Comp. types can group multiple values into one type.

    // TUPLE TYPE
    /* A tuple is a general way of grouping together a number of values with
     a variety of types into one compound type.
     Tuples have a fixed length: once declared, they cannot grow or shrink in size.*/
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // to get the individual values we can use pattern matching
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // we can also access using .X
    let five_hundred = tup.0;
    let one = tup.2;

    // ARRAY TYPE
    // Unlike a tuple, every element of an array must have the same type.
    // arrays in Rust have a fixed length.
    // Arrays are useful when you want your data allocated on the stack rather than the heap.
    let a = [1, 2, 3, 4, 5];
    // VECTOR
    // A vector is a similar collection type provided by the standard library
    // that is allowed to grow or shrink in size.
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [6, 7, 8, 9, 10];
    let a = [3, 5]; // == let a = [3, 3, 3, 3, 3];
    let first  = a[0];
    let second = a[1];




}
