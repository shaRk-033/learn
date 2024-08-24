#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u8,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn build_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
//associated functons

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
// fn calculate_area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
fn main() {
    let u1: User = build_user(String::from("user1"), String::from("user1@xyz.com"));
    println!(
        "User: {}, Email: {}, Sign in count: {}, Active: {}",
        u1.username, u1.email, u1.sign_in_count, u1.active
    );
    let r1: Rectangle = build_rectangle(30, 40);
    let s1 = Rectangle::square(20);
    println!("{s1:#?}");
    println!("Rectangle: width: {}, height: {}", r1.width, r1.height);
    println!("Area of rectangle: {}", r1.area());
    println!("{r1:#?}");
}
