use std::fs::File;
use std::io::ErrorKind;

fn main() {
    /*
    2 types of errors
    recoverable - type Result<T, E>
    unrecoverable - panic!
    */
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
    // more concise
    let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// propagating errors
fn read_username_from_file() -> Result<String, io:Error> {
    let mut username = String::new();
    // "?" operator
    // if Ok, Ok will be returned from the expression
    // if Err, Err will be returned from the whole function 
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
