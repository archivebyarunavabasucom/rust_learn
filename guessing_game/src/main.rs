use rand::Rng;
use colored::*;
use std::{cmp::Ordering, io};
fn main(){
    loop{

    let number = rand::thread_rng().gen_range(10..100);
    println!("the number is {}", number);    
    println!("guess the number");
    let mut guess = String::new();
    // input the number
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to load");
    // trim 
    let guess:u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_)=>continue,
    };
    
    match guess.cmp(&number) {
        Ordering::Less=>println!("{}","less".red()),
        Ordering::Equal=>{
            println!("{}","equal".green());
            break;
        },
        Ordering::Greater=>println!("{}","greater".yellow()),

    }}
    // println!("{}",guess);
}