use std::io;

fn one_serialize() {
    println!("<=Serialize=>");
    println!("Type number in");
    let mut userInput =  String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Fail to read from stdin");
    
    let numericInput = userInput.trim();
    match numericInput.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", numericInput),
    };
}

fn two_deserialize() {
    println!("<=Deserialize=>");
    println!("Type number in");
    let mut userInput =  String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Fail to read from stdin");
    
    let numericInput = userInput.trim();
    match numericInput.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", numericInput),
    };
}

fn three_explanation() {
    println!("The IEEE 754 is a standard for representing floating-point numbers (floating-point numbers) in the binary system in computers.
It defines the representation of real numbers by sign, exponent and mantissa.");
}

fn mainMenu() {
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Fehler beim Einlesen");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ungültige Eingabe. Bitte wähle eine der verfügbaren Optionen.");
                continue;
            }
        };
        match choice {
            1 => {
                one_serialize();
            }
            2 => {
                two_deserialize();
            }   
            3 => {
                println!("<=Explanation=>");
                three_explanation();
            }
            4 => {
                four_help();
            }
            5 => {
                break;
            }
            _ => println!("Wrong input!")
        }
    }
}

fn four_help() {
    println!("<=IEEE.754 Converter=>");
    println!("<=Choose a operation=>");
    println!("<-------------------->");
    println!("<-1.Serialize to 754->");
    println!("<-2.Deserialize     ->");
    println!("<-3.Explanation     ->");
    println!("<-4.Help            ->");
    println!("<-5.Quit            ->");
    println!("<-------------------->");
}
fn main() {
        four_help();
        mainMenu();
}