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
    

        println!("
            1. Study
            2. Work
            3. Party
            4. Gym
            5. Do nothing 
        To quit press Control + C on your keyboard");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        age += 1;

        match input.trim() {
            "1" => {
                happiness -= 5;
                println!("You studied!");
            }

            "2" => {
                money += 10;
                happiness -=5;
                println!("You worked");
            }

            "3" => {
                happiness += 10;
                health -= 5;
                println!("You partied");
            }
            "4" => {
                health += 10;
                println!("You trained");
            }

            "5" => {
                println!("You did nothing");
            }
        
            "6" => {
                println!("Baiiiiii");
                break; // ✅ actually exits now
            }

            _ => println!("Invalid man try again"),
        }

        // Random Events
        let event = rng.gen_range(0..100);

        // lucky event
        if event < 5 {
            money += 20;
            println!("You got money!!");
        }

        // Sickness
        if event >= 5 && event < 10 {
            health -= 10;
            println!("YOu got sick oof");
        }

        // Game over stuff
        if health <= 0 {
            println!("Ye died");
            break;
        }   

        if happiness <= 0 {
            println!("You gave up on life RIP");
            println!("Ye died");
            break;
        }

        if age >= 100 {
            println!("You live a century YAY!!!");
            println!("Ye died");
            break;
        }
    }    
}