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
    0. Age
    1. Study
    2. Work
    3. Party
    4. Gym
    5. Do nothing 
    6. Quit");

    let mut input = String::new();
    io::stdin().readline