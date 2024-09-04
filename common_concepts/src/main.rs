fn main() {
    // variables
    let x = 5; // immutable by default
    let mut x = 5;
    x = 6;
    
    // constants - uppercase, separated by underscores, type annotation REQUIRED
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let x = 5;
    let x = x + 1;
    { // block
        let x = x * 2;
        println!("The value of the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    /* 
    data types
        scalar - represents a single value
        scalar types:
            integers
                i/u 8/16/32/64/128
                i - positive and negative
                u - positive only
                i32 by default
                -(2^n-1) to 2^(n-1)-1
                can write integer literals (decimal/hex/octal/binary/byte)
            floating-point numbers
            booleans
            characters
        compound types:
            tuples
            arrays
    */

    // floating-point type
    let x: f64 = 2.0; // f64 by default

    // numeric operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.1 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    // boolean type
    let t: bool = true;

    // character type
    let z: char = 'Z'; // single quotes

    // tuple type - fixed length, can contain mixed types
    // unit - tuple without any values
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    // accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array type - fixed length, must contain only one type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3, 5]; // [3, 3, 3, 3, 3]
    // accessing array elements
    let first = a[0];
    let second = a[1];

    // functions
    let returned_value = another_function(5, 'h'); // parameter/argument
    println!("{returned_value}");

    // statement - performs some action and does not return a value
    // expression - evaulates to a resultant value, can be part of statements
    let y /* statement */ = { // expression
        let x = 3;
        x + 1 // doesn't need semicolon
        // if semicolon was added, it will turn the expression to a statement (doesn't return a value)
    };
    println!("The value of y is: {y}");

    // if expressions
    let number = if true { 3 } else { 6 }; // arms must have the same type
    if number < 5 { // arm
        println!("number is less than 5");
    } else if number > 5 { // arm
        println!("number is greater than 5");
    } else { // arm
        println!("else expression");
    }

    // loops
    let mut counter = 0;
    'counting_up: loop { // loop label
        println!("counter = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("Counter: {counter}");

    // while loops
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // for loops
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // safer code of the while code
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, unit_label: char) -> i32 { // type declaration REQUIRED
    println!("Another function, passed args: {x} {unit_label}");
    // explicit return
    //return 5;
    5 // implcit return
}
