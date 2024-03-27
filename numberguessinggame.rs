use std::Io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Welcome to the Guessing Game!");
    let secret_number = rand::thread_rng().gren_range(1..101);
    loop{
        println!("Please guess a number between 1 and 100: ");
        let mut guess = String::new();
        Io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println("Too big!"),
            Ordering::Equal=> {
                println!("Congratulations! You win!");
                break;
            }
        }
    }
}