use std::io::{self, Write};
use rand::seq::SliceRandom;

#[derive(Debug,Clone)]
struct Flashcard {
    question: String,
    answer: String,
}

fn input() -> String {
    println!("\nPlease enter your choice:");
    println!("1. Add flashcard");
    println!("2. Start flashcard");
    println!("3. Exit");
    let mut chose = String::new();
    io::stdin().read_line(&mut chose).expect("Failed to read line");
    chose.trim().to_string()
}

fn input_i32() -> i32 {
    loop {
        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");

        match value.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("❌ Invalid input. Please enter a valid number:");
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn input_string() -> String {
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    value.trim().to_string()
}

fn main() {
    let mut flashcard: Vec<Flashcard> = Vec::new();

    loop {
        let choice = input();

        if choice == "1" {
            println!("Please enter the number of questions you want to add:");
            let num = input_i32();

            for i in 0..num {
                println!("Please enter question {}:", i + 1);
                let question = input_string();
                println!("Please enter the answer for question {}:", i + 1);
                let answer = input_string();

                flashcard.push(Flashcard {question,answer,});
            }

        } else if choice == "2" {
            if flashcard.is_empty() {
                println!("⚠️ No flashcards available. Please add some first!");
                continue;
            }

           
            let mut rng = rand::thread_rng();
            let mut shuffled_flashcards = flashcard.clone();
            shuffled_flashcards.shuffle(&mut rng);

            for (i, card) in shuffled_flashcards.iter().enumerate() {
                println!("\nQuestion {}: {}", i + 1, card.question);
                let answer = input_string();

                if answer == card.answer {
                    println!("✅ Correct!");
                } else {
                    println!("❌ Incorrect\nCorrect answer is: {}", card.answer);
                }
            }

        } else if choice == "3" {
            println!("👋 Exiting the program. Goodbye!");
            break;
        }
    }
}
