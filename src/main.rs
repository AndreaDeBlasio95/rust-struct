// STRUCTS
// REMEMBER: we use & if we don't want to take ownership of the struct
// Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. Methods let you specify the behavior that instances of your structs have, and associated functions let you namespace functionality that is particular to your struct without having an instance available.

fn main() {
    
    // Creating an instance of a struct immutable
    let user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    };

    // Creating an instance of a struct mutable
    let mut user2 = User {
        email: String::from("example@gmail.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1, 
    };

    user2.email = String::from("newemail@gmail.com");

    let user3 = User {
        email: String::from("emailhere@gmail.com"),
        username: String::from("usernamehere"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("emailhere@gmail.com"),
        username: String::from("usernamehere"),
        ..user1
    };
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is: {}", area(&rect1));
    
    // printing the entire struct with println!()
    println!("The rect1 is: {:#?}", rect1);

    // Using the method syntax
    println!("The area of the rectangle using method syntax is: {}", rect1.area());
}


// Defining a struct
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

// derive(Debug) allows us to print the struct with println!()
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Method Syntax    ----------------------------
// self is a reference to the instance of the struct
// Each struct can have multiple impl blocks
impl Rectangle {
    // Methods are defined within the context of a struct
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

/*
fn build_user(email: String, username: String) -> User {
    User {
        email,      // Field init shorthand, same as email: email
        username,   // Field init shorthand, same as username: username
        active: true,
        sign_in_count: 1,
    }
}
*/

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}