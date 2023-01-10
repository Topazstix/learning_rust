
use rand::{Rng};



fn main() {
    println!("Hello, world!");

    let mut random_array: Vec<u32> = Vec::new();
    for x in 0..20 {
        random_array.push(rand::thread_rng().gen_range(1..99999));
    }

    for item in random_array{
        println!("{}",item);
    }
    
}