/* 
ownership rules:
- each value has an owner
- there can be only one owner at a time
- when the owner gets out of scope, the value will be dropped
*/
fn main() {
    {                           // s is not valid here, since it's not yet declared
        let s = "hello";  // s is valid from this point forward
        // do stuff with s
    }                           // this scope is now over, and s is no longer valid
    
    // String type - stores an amount of text unknown at compile time
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{s1}");
    // deep copy - copies heap data as well
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // doesnt need clone() because integers have a known size at compile time
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s = String::from("hello");  // s comes into scope
    take_ownership(s);              // s's value moves into the function and is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function.
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    references();
    mutable_references();
    slices();
} // <- x goes out of scope, then s
  // however, s's value was moved so nothing happens

fn take_ownership(some_string: String) {    // some_string comes into scope
    println!("{some_string}");
} // <- some_string goes out of scope and `drop` is called
  // the backing memory is freed

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{some_integer}");
} // some_integer goes out of scope
  // nothing happens

// reference (&) - only refers to the value but not own it
fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // <- s goes out of scope, but it doesn't have ownership of s, so it's not dropped

// references are immutable by default
fn mutable_references() {
    let mut s = String::from("hello");
    change(&mut s);
    // can only have one reference to a mutable value
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM
    //println!("{r1},{r2},{r3}");
    println!("{r1},{r2}");
    // r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn slices() {
    let my_string = String::from("hello world");
    // first_word works on slices of String's whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // first_word also works on references to String's
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    // because string literals are string slices already, it can be passed directly
    let word = first_word(my_string_literal);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
// returns length of first word in a string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // string slice
        }
    }

    &s
}
