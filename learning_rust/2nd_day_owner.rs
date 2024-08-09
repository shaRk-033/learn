
#[derive(Debug)]
struct User{
    username : String,
    email : String,
    active : bool,
    sign_in_count : i8,
}
impl User{
    fn eligible_for_discount(&self) -> bool{
        return self.sign_in_count > 5;
    }
}
fn construct_user(username : String, email : String) -> User{
    User{
        username,
        email,
        active : true,
        sign_in_count : 10,
    }
}
fn print_user(user : &User){
    print!("Username: {}\nEmail: {}\nActive: {}\nSign in count: {}", user.username, user.email, user.active, user.sign_in_count);
}

fn main(){
    let s1: String = String::from("hello world my name is rakesh");
    let l: [i32; 5] = [1, 2, 3, 4, 5];
    let user1: User = construct_user(String::from("rakesh"), String::from("abc"));

    // print_user(user1);// notice how we gave up the ownership of user1 to the fucntion
    // print!("{}", user1.active);// so when we try to access it it doesnt work because we no longer own it
    //so instead of passing the object itself we pass the reference, ie we are just letting the fucntion "borrow" the object

    // some notes :
    // there cant be two owners of an object
    // a = b 
    // two cases arise here:
    // 1. a is a primitive type, in this case the value of b is copied to a. b is declared on the stack ie has fixed memory allocated
    // 2. b is allocated on the heap, so when perform a = b, both a and b point to the same memory location. This cant happen because deallocatng the memory of a will deallocate the memory of b as well.
    // so in rust, when we do a = b, b is moved to a. This means that b is no longer valid and a is the owner of the object. This is called ownership in rust. buzzinga

    // print_user(&user1);
    println!("{user1:#?}");
    println!("{}", user1.eligible_for_discount());
}

// fn first_word(s : &str) -> &str{
//     let bytes: &[u8] = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     return &s[..];
// }
