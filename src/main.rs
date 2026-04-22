use std::io;
use rand::Rng;
 
fn main() {
    let mut age = 0;
    let mut health = 100;
    let mut happiness = 80;
    let mut money = 0;

    let mut rng = rand::thread_rng();

    println!("You are born! Your journey begins.");

    loop {
        println!("\n====================");
        println!("Age: {}", age);
        println!("Health: {} | Happiness: {} | Money: {}", health, happiness, money);
        println!("====================");
    }

    println!("
    1. Study
    2. Work
    3. Party
    4. Gym
    5. Do nothing 
    6. Quit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    age += 1;

    match input.trim() {
        "1" => {
            happiness -= 5;
            println!("You studied!")
        }

        "2" => {
            money += 10;
            happiness -=5;
            println!()
        }
        
    }