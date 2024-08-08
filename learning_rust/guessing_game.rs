use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("enter a number");
    
    let random = rand::thread_rng().gen_range(1..=100);
    
    loop{
        let mut number = String::new();
        io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

        let number : u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match number.cmp(&random){
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("u win");
                break;
            }
        }
    }
}
