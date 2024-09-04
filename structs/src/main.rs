// like tuples, but can name each piece of data
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
// methods - unlike functions, they are defined within the context of a struct
impl Rectangle {
    // first parameter is always self - represents the instance of the stuct
    // also short for `self: &Self`
    // associated functions - all functions defined within an impl block
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Self - alias for the type that appears after the impl keyword (Rectangle)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = build_user(
        String::from("someusername123"), 
        String::from("someone@example.com")
    );
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // .. syntax
    };

    // both values have different types even if each struct has the same values
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: dbg!(50 * scale),
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // for printing entire stuct
    println!("rect1 is {:?}", rect1); // {:#?} for pretty-print
    dbg!(&rect1);

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    dbg!(&sq);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1
    }
}
