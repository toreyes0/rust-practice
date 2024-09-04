
// enumerates possible variants of a type
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Rust doesn't have nulls, but ut does have an enum that can 
// encode the concept of a value being present or absent - the Option<T> enum
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');
    // this somehow gets an error now
    //let absent_number: Option<i32> = None;

    // if rolling anything other than 3 or 7, reroll
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // placeholder
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
