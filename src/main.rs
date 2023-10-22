use std::io;

fn three_explanation() {
    println!("The IEEE 754 is a standard for representing floating-point numbers (floating-point numbers) in the binary system in computers.\n
    It defines the representation of real numbers by sign, exponent and mantissa.");
}

fn mainMenu() {
    let mut choice = String::new();
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
            3 => {
                three_explanation();
            }
            4 => {
                break;
            }
            _ => println!("Wrong input!")
        }
    }
}

fn main() {
        println!("<=IEEE.754 Converter=>");
        println!("<=Choose a operation=>");
        println!("<-------------------->");
        println!("<-1.Serialize to 754->");
        println!("<-2.Deserialize     ->");
        println!("<-3.Explanation     ->");
        println!("<-4.Quit            ->");
        println!("<-------------------->");

        mainMenu();
}