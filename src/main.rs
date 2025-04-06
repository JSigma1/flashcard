use std::io::{self,Write};
#[derive(Debug)]
struct Flashcard{
    question : String,
    answer : String,
}

fn input() ->  String{
    println!("\nPlease enter your choice");
    println!("1.Add flashcard");
    println!("2.Start flashcard");
    println!("3.Exit");
    let mut  chose = String::new();
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
                print!("> ");
                io::stdout().flush().unwrap();
            }
        }
    }
}


fn input_string() -> String{
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("fail to read line");
    value.trim().to_string()
}

fn main(){
    let mut flashcard :Vec<Flashcard>= Vec::new();

    loop {
        let  choice = input();
        if choice == "1".to_string(){
            println!("Please enter number of questions you want to add");
            let  num = input_i32();
            for i in 0..num {
                println!("Please enter question {}", i+1);
                let mut question = String::new();
                io::stdin().read_line(&mut question).expect("Failed to read line");
                println!("Please enter answer for question {}", i+1);
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Failed to read line");
                flashcard.push(Flashcard{question : question.trim().to_string(),answer :  answer.trim().to_string()});
            }
            
        }else if choice =="2".to_string(){
            for i in 0..flashcard.len(){
                println!("Question {} : {} ",i+1,flashcard[i].question);
                let answer = input_string();
                if answer == flashcard[i].answer{
                    println!("✅Correct");
                }else {
                    println!("❌ Incorrect
Correct answer is : {}", flashcard[i].answer);
                }
            }
        }else {
            println!("Exiting the program");
            break;
        }

    }
}